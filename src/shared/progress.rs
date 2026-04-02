use indicatif::{ProgressBar, ProgressStyle};

pub struct BarraProgreso {
    barra: Option<ProgressBar>,
}

impl BarraProgreso {
    pub fn new(total: usize) -> Self {
        if total == 0 {
            return Self { barra: None };
        }

        let barra = ProgressBar::new(total as u64);
        barra.set_style(
            ProgressStyle::with_template("{spinner:.cyan} [{bar:40}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏▘▙▚▛▜▝▞▟"),
        );
        barra.set_message("Moviendo archivos...");

        Self { barra: Some(barra) }
    }

    pub fn incremento(&self) {
        if let Some(ref barra) = self.barra {
            barra.inc(1);
        }
    }

    pub fn finalizar(self) {
        if let Some(barra) = self.barra {
            barra.finish();
        }
    }
}
