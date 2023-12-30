def xor_file(file_path, xor_value):
    with open(file_path, 'rb') as file:
        content = file.read()

    xor_value = int(xor_value, 16)

    xored_content = bytes([byte ^ xor_value for byte in content])

    with open(f"{file_path}.out", 'wb') as output_file:
        output_file.write(xored_content)

if __name__ == "__main__":
    file_path = "some filename"  
    # 0x30
    hex_value = "30"  

    xor_file(file_path, hex_value)
