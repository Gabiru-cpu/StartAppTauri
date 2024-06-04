use serde::Deserialize;

#[derive(Deserialize)]
pub struct TesteData {
    nome: String,
    idade: i32,
}

impl TesteData {
    // Getter para 'nome'
    pub fn nome(&self) -> &str {
        &self.nome
    }

    // Setter para 'nome'
    #[allow(dead_code)]
    pub fn set_nome(&mut self, nome: String) {
        self.nome = nome;
    }

    // Getter para 'idade'
    pub fn idade(&self) -> i32 {
        self.idade
    }

    // Setter para 'idade'
    #[allow(dead_code)]
    pub fn set_idade(&mut self, idade: i32) {
        self.idade = idade;
    }
}