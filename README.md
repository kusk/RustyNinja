# RustyNinja - A stealthy file copy
>[!WARNING]
>Beware this is really really shitty and ugly code that by some miracle works. Basicly a learn rust project for me.

A (in spirit) rust version of NinjaCopy(https://github.com/PowerShellMafia/PowerSploit/blob/master/Exfiltration/Invoke-NinjaCopy.ps1).

This tool lets you, if you have **Admin privileges**, access and copy otherwise locked files like registry database files(SYSTEM, SAM, SECURITY) or NTDS.dit. 

>[!TIP]
>Accessing registry databases and NTDS.dit this way may(might/hopefully/test it goddamn it!) bypass EDR/detection which would be lovely during red team assignments. Other methods such as using ntdsutil, disk shadow, reg save or via RPC can be way too **noisy**.


I have added an additional detection bypass which XOR's the file content in memory before writing it to desk. The output filename has a randomly generated filename just in case. 

If you don't want to XOR the file content for some reason, just give it 0x00 as the XOR byte.

The program takes two arguments.

Using RustyNinja:
~~~rust
rustyninja.exe [path to file to copy] [hex value to Xor data with]

Example: 

C:\Users\a\rustyninja.exe c:\windows\system32\config\SYSTEM 0x33

Xoring and saving 20185088 bytes of data to: tySXLjFMRndoxTuq
~~~

Compiling:
~~~rust
cargo build --release
~~~

Big thanks and inspiration from Colin Finck who created the NTFS implementation in Rust.
