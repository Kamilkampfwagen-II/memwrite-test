use memwrite_test::*;


fn main() {
    let pid: u32 = 6664;
    let address: u64 = 0x1AB6761CCD0; // tonemap grading (Float3)

    let process_handle = open_process(pid).expect("Failed to open process!");

    let color = Float3 {x: 0.5, y: 0.5, z: 0.5 };
    write_value_to_address(process_handle, address, color).expect("Failed to write process memory!");

    let read_value: Float3 = read_value_from_address(process_handle, address).expect("Failed to read process memory!");
    println!("{:?}", read_value)
}