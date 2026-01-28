use pyo3::prelude::*;
use std::collections::VecDeque;

#[pyclass]
pub struct MicrostructureAnalyzer {
    volume_buckets: VecDeque<f64>,
    buy_volume_buckets: VecDeque<f64>,
    bucket_size: f64,
    max_buckets: usize,
}

#[pymethods]
impl MicrostructureAnalyzer {
    #[new]
    fn new(bucket_size: f64, max_buckets: usize) -> Self {
        MicrostructureAnalyzer {
            volume_buckets: VecDeque::with_capacity(max_buckets),
            buy_volume_buckets: VecDeque::with_capacity(max_buckets),
            bucket_size,
            max_buckets,
        }
    }

    // Calcula el VPIN (Toxicidad del Flujo)
    // Nota: '_price' tiene un guion bajo para indicar que no se usa por ahora (future-proof)
    fn calculate_vpin(&mut self, _price: f64, volume: f64, side: i32) -> f64 {
        // Lógica simplificada de VPIN para HFT
        let buy_vol = if side == 0 { volume } else { 0.0 }; // 0 es BUY en Aether
        
        self.volume_buckets.push_back(volume);
        self.buy_volume_buckets.push_back(buy_vol);

        if self.volume_buckets.len() > self.max_buckets {
            self.volume_buckets.pop_front();
            self.buy_volume_buckets.pop_front();
        }

        if self.volume_buckets.is_empty() { return 0.0; }

        let total_vol: f64 = self.volume_buckets.iter().sum();
        let mut oi_sum: f64 = 0.0;

        for i in 0..self.volume_buckets.len() {
            let sell_vol = self.volume_buckets[i] - self.buy_volume_buckets[i];
            oi_sum += (self.buy_volume_buckets[i] - sell_vol).abs();
        }

        if total_vol > 0.0 {
            oi_sum / total_vol // Retorna el VPIN (0.0 a 1.0)
        } else {
            0.0
        }
    }
}

// --- CORRECCIÓN CRÍTICA PARA PYO3 0.23+ ---
// Usamos el tipo Bound<'_, PyModule> en lugar de &PyModule
#[pymodule]
fn aether_fast_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MicrostructureAnalyzer>()?;
    Ok(())
}