//! ID generation strategies.

/// A trait for generating IDs.
/// This is used in memory generation as each memory generally needs to be assigned an ID (since not all storage types will come with their own ID generation).
pub trait IdGenerationStrategy {
    fn generate_id(&mut self) -> String;
}

/// A struct that randomly generates UUID V4 IDs.
#[cfg(feature = "uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
pub struct UuidV4Generator;

#[cfg(feature = "uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
impl IdGenerationStrategy for UuidV4Generator {
    fn generate_id(&mut self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

/// A counter. Provides IDs as numbers starting from 1 by default.
pub struct Counter(u64);

impl Counter {
    /// Creates a new instance of a counter. Starts from 1.
    pub fn new() -> Self {
        Self(1)
    }

    /// Initialises a counter with a given number.
    pub fn from_number(num: u64) -> Self {
        Self(num)
    }

    pub fn get_id(&mut self) -> u64 {
        let num = self.0;
        self.0 += 1;
        num
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerationStrategy for Counter {
    fn generate_id(&mut self) -> String {
        self.get_id().to_string()
    }
}

/// A generic ID memory generator. Creates IDs in the format `<foo>-<number>`. Uses [`Counter`] internally for ID incrementing.
pub struct MemoryIdGenerator {
    prefix: String,
    counter: Counter,
}

impl MemoryIdGenerator {
    pub fn new() -> Self {
        Self {
            prefix: "mem".to_string(),
            counter: Counter::new(),
        }
    }

    pub fn builder() -> MemoryIdGeneratorBuilder {
        MemoryIdGeneratorBuilder::new()
    }
}

impl Default for MemoryIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerationStrategy for MemoryIdGenerator {
    fn generate_id(&mut self) -> String {
        // This should only error out at NaN or wrapping
        let id = self.counter.generate_id().parse::<u64>().unwrap();
        format!("{prefix}-{id:09}", prefix = self.prefix)
    }
}

/// A builder instance for [`MemoryIdGenerator`].
pub struct MemoryIdGeneratorBuilder {
    prefix: Option<String>,
    counter: Option<Counter>,
}

impl MemoryIdGeneratorBuilder {
    /// Create a new instance of [`MemoryIdGeneratorBuilder`]
    pub fn new() -> Self {
        Self {
            prefix: None,
            counter: None,
        }
    }

    /// Sets a new prefix (ie, "doc" will output an ID of "doc-000001 if the counter is 1")
    pub fn prefix<S>(mut self, prefix: S) -> Self
    where
        S: AsRef<str>,
    {
        self.prefix = Some(prefix.as_ref().to_string());
        self
    }

    /// Add a counter.
    pub fn counter(mut self, counter: Counter) -> Self {
        self.counter = Some(counter);
        self
    }

    /// Build the Memory ID generator.
    pub fn build(self) -> MemoryIdGenerator {
        let prefix = self.prefix.unwrap_or("mem".to_string());
        let counter = self.counter.unwrap_or_default();

        MemoryIdGenerator { prefix, counter }
    }
}

impl Default for MemoryIdGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::id_gen::IdGenerationStrategy;
    use crate::id_gen::MemoryIdGenerator;

    #[test]
    fn test_id_gen_works() {
        let mut generator = MemoryIdGenerator::new();
        let id = generator.generate_id();

        assert_eq!("mem-000001", &id);

        let id = generator.generate_id();

        assert_eq!("mem-000002", &id);
    }
}
