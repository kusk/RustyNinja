mod sector_reader;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, Write};
use anyhow::{bail, Context, Result};
use ntfs::indexes::NtfsFileNameIndex;
use ntfs::{Ntfs, NtfsFile, NtfsReadSeek};
use sector_reader::SectorReader;
use rand::distributions::{Alphanumeric, DistString};
extern crate ntfs;
extern crate rand;
extern crate anyhow;

struct CommandInfo<'n, T>
where
    T: Read + Seek,
{
    current_directory: Vec<NtfsFile<'n>>,
    fs: T,
    ntfs: &'n Ntfs,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut dirs: Vec<&str> = path.split('\\').collect();

    let f: File = File::open([r"\\.\", dirs[0]].concat())?;
    let sr = SectorReader::new(f, 4096)?;
    let mut fs = BufReader::new(sr);
    let mut ntfs = Ntfs::new(&mut fs)?;
    ntfs.read_upcase_table(&mut fs)?;
    let current_directory = vec![ntfs.root_directory(&mut fs)?];
    let mut info = CommandInfo {current_directory,fs,ntfs: &ntfs};

    let target_file = dirs.pop();
    for dir in dirs.iter().skip(1) {
            cd(dir, &mut info)?;
    }
    let key = &args[2];
    get(target_file.to_owned().unwrap(), &mut info, key.to_owned())?;

    Ok(())
}

fn cd<T>(arg: &str, info: &mut CommandInfo<T>) -> Result<()>
where
    T: Read + Seek,
{
        let index = info.current_directory.last().unwrap().directory_index(&mut info.fs)?;
        let mut finder = index.finder();
        let maybe_entry = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg);
        let entry = maybe_entry.unwrap()?;
        let _file_name = entry.key().expect("key must exist for a found Index Entry")?;
        let file = entry.to_file(info.ntfs, &mut info.fs)?;
        info.current_directory.push(file);
    Ok(())
}


fn get<T>(arg: &str, info: &mut CommandInfo<T>, key: String) -> Result<()>
where
    T: Read + Seek,
{
    let out_file = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let (file_name, data_stream_name) = match arg.find(':') {
        Some(mid) => (&arg[..mid], &arg[mid + 1..]),
        None => (arg, ""),
    };

    let mut output_file = OpenOptions::new().write(true).create_new(true).open(&out_file).with_context(|| format!("Failed to open file for writing"))?;
    let file = filearg(file_name, info)?;
    

    let data_item = match file.data(&mut info.fs, data_stream_name) {
        Some(data_item) => data_item,
        None => {
            println!("Missing $DATA");
            return Ok(());
        }
    };

    let data_item = data_item?;
    let data_attribute = data_item.to_attribute()?;
    let mut data_value = data_attribute.value(&mut info.fs)?;

    println!("Xoring and saving {} bytes of data to: {}", data_value.len(), out_file);

    let mut buf = [0u8; 4096];
    if let Ok(xorkey) = u8::from_str_radix(key.trim_start_matches("0x"), 16) {
        loop {
            let bytes_read = data_value.read(&mut info.fs, &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            for i in 0..bytes_read {
                buf[i] ^= xorkey;
            }
            output_file.write_all(&buf[..bytes_read])?;
        }
    } else {
        println!("Couldn't xor the data.");
    }
    Ok(())
}

#[allow(clippy::from_str_radix_10)]
fn filearg<'n, T>(arg: &str, info: &mut CommandInfo<'n, T>) -> Result<NtfsFile<'n>>
where
    T: Read + Seek,
{
        let index = info.current_directory.last().unwrap().directory_index(&mut info.fs)?;
        let mut finder = index.finder();

        if let Some(entry) = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg) {
            let entry = entry?;
            let file = entry.to_file(info.ntfs, &mut info.fs)?;
            Ok(file)
        } else {bail!("File not found: \"{}\".", arg)}
}
