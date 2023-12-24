use std::io;

const  UPPERCASE_ALF: [char; 26]= ['A', 'B', 'C', 'D', 'E', 'F', 'G','H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X','Y', 'Z'];

const  LOWERCASE_ALF: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    println!("Olá sou programa que mexe com a Cifra de César, comigo você  pode tanto criptografar como descriptografar mensagens.\n\
    Informe uma das seguintes opções:\n\
    Criptografar\n\
    Descriptografar\n\
    ");

    let mut opc_crip = String::new();

    io::stdin()
    .read_line(&mut opc_crip)
    .expect("Erro ao guardar a opção do usuário!");

    match opc_crip.trim()
    {
        "Criptografar" =>
        {
            println!("Informe a mensagem que você quer criptografar (ela não pode conter caracteres especiais):");

            let mut message = String::new();

            io::stdin()
            .read_line(&mut message)
            .expect("Erro ao pegar a mensagem para criptografar!");

            println!("Informe quantas letras o vão ser deslocadas no alfabeto:");

            let mut desloca_input = String::new();

            io::stdin()
            .read_line(&mut desloca_input)
            .expect("Erro ao guardar o deslocamento do usuário!");

            let message_encrypt = encrypt(&message, &desloca_input);

            println!("A mensagem criptografada fica assim: {}", message_encrypt);
        },
        "Descriptografar" =>
        {
            println!("Informe a mensagem que você quer descriptografar (ela não pode conter caracteres especiais):");

            let mut message = String::new();

            io::stdin()
            .read_line(&mut message)
            .expect("Erro ao pegar a mensagem para descriptografar!");

            println!("Informe quantas letras foram deslocadas no alfabeto:");

            let mut desloca_input = String::new();

            io::stdin()
            .read_line(&mut desloca_input)
            .expect("Erro ao guardar a quantidade deslocada!");

            let message_decrypt = decrypt(&message, &desloca_input);

            println!("A mensagem descriptografada fica assim: {}", message_decrypt);
        }
        _=>
        {
            println!("Erro: Digite uma opção valida!");
        }
    }
}


fn encrypt(msg:&str, desl:&str ) -> String
    {
        let mut result = String::new();

        // pegando cada letra da variável msg por completo com o 'lattes'

        for lattes in msg.chars()
        {
            // Transformando o 'desloca' num número.
            let desl_num = match desl.trim().parse::<i32>()
            {
                Ok(num) => num,
                Err(_) =>
                {
                    println!("Erro: Informe um número valido!");
                    break
                }
            };

            // Verificando se a letra é minuscula ou maiúscula
            if lattes.is_alphabetic()
            {
            match lattes.is_ascii_lowercase()
            {
                true =>
                {
                    for index in 0..LOWERCASE_ALF.len()
                    {
                        
                        if lattes == LOWERCASE_ALF[index]
                        {
                            let soma: i32= (index as i32 + desl_num) %26 ;

                            result.push( LOWERCASE_ALF[soma as usize]);
                        }
                    }
                    
                },
                false=>
                {
                    for index in 0..UPPERCASE_ALF.len()
                    {
                        if lattes == UPPERCASE_ALF[index]
                        {
                            let soma:i32 = (index as i32 + desl_num) %26 ;

                            result.push( UPPERCASE_ALF[soma as usize]);
                        }
                       
                    }

                }
            }
            }
            else
            {
                result.push(lattes);
            }
        }
        result.trim().to_string()
}


    // Descriptografando a mensagem.

fn decrypt(msg: &str, num_decrypt: &str) -> String
{
    let mut result = String::new();

    for lattes in msg.chars()
    {
         let desl_num = match num_decrypt.trim().parse::<i32>()
         {
             Ok(num) => num,
             Err(_) => 
             {
                println!("Erro: Informe um número!");
                break
             }
         };

         if lattes.is_alphabetic()
         {
            match lattes.is_ascii_lowercase()
             {
                true =>
                {
                    for index in 0..LOWERCASE_ALF.len()
                    {
                        if lattes == LOWERCASE_ALF[index]
                        {
                            let sub: i32 = (index as i32 - desl_num + 26) %26;

                            result.push( LOWERCASE_ALF[sub as usize]);
                        }
                    }
                },
                false =>
                {
                    for index in 0..UPPERCASE_ALF.len()
                    {
                        if lattes == UPPERCASE_ALF[index]
                        {
                            let sub: i32 = (index as i32 - desl_num + 26) %26;
                            
                            result.push(UPPERCASE_ALF[sub as usize]);
                        }
                    }
                }
             }
         }
         else
         {
             result.push(lattes);
         }
    }
    result.trim().to_string()
}
