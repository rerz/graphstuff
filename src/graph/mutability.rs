pub trait Mutability {}

pub struct Mutable;

impl Mutability for Mutable {}

pub struct Immutable;

impl Mutability for Immutable {}
