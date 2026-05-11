#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RuleViolationType {
    SingleLocation,
    SingleObject,
    Cycle,
    IncompleteLayerSpecification,
    LayerDoNotExist,
    DeclaredLayerNotFound,
}
