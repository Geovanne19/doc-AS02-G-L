pub fn ler_e_inserir_conjunto(&mut self, texto: &str, tolerancia: u8){
    let mut entrada: String = String::new();
    let mut contador: u8 = 0;    
    while contador < tolerancia{
        entrada.clear();
        println!("{}", texto);
        match stdin().read_line(&mut entrada) {
            Ok(_) =>{
                match self.auxilia_leitura_de_conjunto(&mut entrada) {
                    Some(nova_arvore) =>{
                        self.inserir_conjunto(nova_arvore);
                        break;
                    },
                    None =>{
                        println!("Valor inválido!\n\n");
                        contador += 1;
                    }
                }
            }
            Err(erro) =>{
                contador += 1;
                println!("Falha ao ler entrada\nErro: {}\n\n", erro);
            }
        }
    }
}
