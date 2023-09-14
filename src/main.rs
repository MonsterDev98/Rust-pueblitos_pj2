fn main() {
    let height: i16 = 43;

    townelection(height);
}


fn townelection (option: i16) -> u8 {
    //Towns of Caldas:
    let h_norcasia: i16 = 700;
    let h_chinchina: i16 = 1382;
    let h_riosucio: i16 = 1783;

    //Towns of Quindío:
    let h_filandia: i16 = 1923;
    let h_salento: i16 = 1895;
    let h_quimbaya: i16 = 1339;

    //Towns of Risaralda:
    let h_lavirginia: i16 = 890;
    let h_santa_rosa: i16 = 1715;
    let h_santuario: i16 = 1565;

    if option >= 500 && option <= 1000{
        println!("Los lugares que tienen esta altura son Norcasia: {}msnm\n La Virginia: {}msnm ", h_norcasia , h_lavirginia);
    } else if option >= 1000 && option <= 1500{
        println!("Los lugares que tienen esta altura son Chinchiná: {}msnm\n Quimbaya: {}msnm", h_chinchina, h_quimbaya);
    } else if option >= 1500 && option <= 2000 {
        println!("Los lugares que tienen esta altura son Santuario : {}msnm\n Santa Rosa: {}msnm\n Riosucio: {}msnm\n Salento: {}msnm\n Filandia: {}msnm", h_santuario, h_santa_rosa, h_riosucio,h_salento,h_filandia);
    } else {
        print!("ERROR! Ingrese una altura entre 500 y 2000 msnm\n");
    }
    return 1;    

}