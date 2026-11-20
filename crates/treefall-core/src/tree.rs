/// Un árbol descrito con los datos mínimos del Hito 0.
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub species: String,
    /// Altura total en metros.
    pub height_m: f64,
    /// Diámetro del tronco a 1.3 m del suelo (DBH), en centímetros.
    pub dbh_cm: f64,
    /// Inclinación del tronco respecto a la vertical, en grados.
    pub lean_deg: f64,
}

impl Tree {
    /// Relación de esbeltez: altura (m) dividida entre diámetro (m).
    ///
    /// Es un indicador clásico en arboricultura: por encima de ~100 el árbol
    /// se considera esbelto y más propenso a fallar con viento.
    ///
    /// EJERCICIO 1 (tuyo): implementa esta función.
    /// Pistas: el DBH viene en centímetros y la altura en metros, así que
    /// tienes que convertir unidades antes de dividir.
    pub fn slenderness(&self) -> f64 {
        todo!("calcula height_m / (dbh_cm en metros)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oak() -> Tree {
        Tree {
            species: "Quercus robur".to_string(),
            height_m: 20.0,
            dbh_cm: 50.0,
            lean_deg: 0.0,
        }
    }

    #[test]
    fn slenderness_of_a_20m_tree_with_50cm_trunk_is_40() {
        assert!((oak().slenderness() - 40.0).abs() < 1e-9);
    }

    #[test]
    fn thinner_trunk_means_more_slender() {
        let thin = Tree { dbh_cm: 25.0, ..oak() };
        assert!(thin.slenderness() > oak().slenderness());
    }
}
