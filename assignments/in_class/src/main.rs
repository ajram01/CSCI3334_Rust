struct ComputingDevice{
    cpu: String,
    ram: u16,
}

impl ComputingDevice{
    fn new(cpu: String, ram: u16) -> ComputingDevice{
        Self{
            cpu: cpu,
            ram: ram
        }
    }

    fn computation(&self){
        println!("The computation 16 / 2 = 8 was performed by my computer with a cpu: {}, and {}gb of ram", self.cpu, self.ram);
    }
}

fn main() {
    let my_computer: ComputingDevice = ComputingDevice::new(
        "AMD".to_string(),
        16
    );

    my_computer.computation();
}
