Código alvo para v0.1 do shift.

Com uma escrita simples e legível, o foco será a redução da curva de desenvolvimento em Rust, criando código de maneira simplificada que será depois transpilado para Rust.

hello.st
```
import { Server } from "http/server"; 
import { json } from "json";
import { read_to_string } from "fs"; //use std::fs

// Um banco de dados em memória simples
let contador = 0; //let contador: i64 = 0;

// O servidor mais simples do mundo
Server.listen(3000, (req) => {
    print("Recebi request: " + req.method);

    if req.url == "/" {
        return "Bem vindo ao Shift!";
    }

    if req.url == "/api/count" {
        contador = contador + 1;
        
        // Retornando JSON automaticamente
        return json.stringify({ 
            "total": contador,
            "status": "ok"
        });
    }

    if req.url == "/config" {
        // Lendo do disco
        let config = read_to_string("./config.txt");
        return config;
    }

    return "404 Not Found";
});

print("Servidor rodando na porta 3000...");' //println!("");
```

*O projeto está em alpha e poderá sofrer alterações em sua arquitetura e sintaxe*