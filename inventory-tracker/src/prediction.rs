use crate::models::VendaHistorico; 
use linfa::prelude::*;
use linfa_linalg::Float;
use ndarray::{Array1, Array2};

pub enum PrevisaoErro {
    DadosInsuficientes,
    ErroTreinamento(String),
}

pub fn regressao_linear(historico: &[VendaHistorico]) -> Result<f64, PrevisaoErro> { 
    if historico.len() < 2 {
        return Err(PrevisaoErro::DadosInsuficientes);
    }

    let x_data: Vec<f64> = (0..historico.len()).map(|i| i as f64).collect(); 
    let y_data: Vec<f64> = historico.iter().map(|v| v.quantidade as f64).collect();
    
    let records = Array2::from_shape_vec((historico.len(), 1), x_data)
        .map_err(|e| PrevisaoErro::ErroTreinamento(format!("Erro ao criar Array X: {}", e)))?; 

    let targets = Array1::from_vec(y_data);

    
    let dataset = Dataset::new(records, targets);

    let modelo = match LinearRegression::default().fit(&dataset) { 
        Ok(m) => m,
        Err(e) => return Err(PrevisaoErro::ErroTreinamento(format!("Erro ao treinar: {}", e))),
    };

    let proximo_tempo = historico.len() as f64;

    let previsao_record = Array2::from_shape_vec((1, 1), vec![proximo_tempo]) 
        .map_err(|e| PrevisaoErro::ErroTreinamento(format!("Erro ao criar Array X de previsao: {}", e)))?;

    let previsao = modelo.predict(previsao_record); 

    Ok(previsao[[0, 0]])
}

pub fn calcular_media_movel(historico: &[VendaHistorico], janelas: usize) -> f64 { 
    let valores: Vec<f64> = historico.iter()
        .map(|v| v.quantidade as f64)
        .collect(); 

    if valores.len() < janelas{
        return 0.0;
    }

    let soma: f64 = valores[valores.len() - janelas..]
        .iter() 
        .sum(); 

    soma / (janelas as f64)

}
