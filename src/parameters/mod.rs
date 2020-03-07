#[derive(Clone)]
pub struct Parameters {
    pub security_zk: u16,
    pub security_soundness: u16,
    pub hash_to_prime_bits: u16, // μ
    pub field_size_bits: u16, // ν
}

quick_error! {
    #[derive(Debug)]
    pub enum ParametersError {
        InvalidParameters {}
    }
}

impl Parameters {
    pub fn from_security_level(security_level: u16) -> Result<Parameters, ParametersError> {

        let parameters = Parameters {
            security_zk: security_level - 3,
            security_soundness: security_level - 2,
            field_size_bits: 2*security_level,
            hash_to_prime_bits: 2*security_level - 2,
        };

        parameters.is_valid()?;
        Ok(parameters)
    }

    pub fn is_valid(&self) -> Result<(), ParametersError> {
        let d = 1 + (self.security_zk + self.security_soundness + 2)/self.hash_to_prime_bits;
        if d*self.hash_to_prime_bits + 2 <= self.field_size_bits {
            Ok(())
        } else {
            Err(ParametersError::InvalidParameters)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parameters;

    #[test]
    fn test_valid_for_128() {
        let params = Parameters::from_security_level(128).unwrap();
        params.is_valid().unwrap();
    }
}
