use sqlx::types::chrono::{DateTime, Utc, Local};

trait ConstraintTrait<T> {
    fn new(column: String, operator: String, value: T) -> Self;
    fn to_sql(&self) -> String;
}

pub struct StringConstraint {
    column: String,
    operator: String,
    value: String
}
impl ConstraintTrait<String> for StringConstraint {
    fn new(column: String, operator: String, value: String) -> Self {
        StringConstraint { column, operator, value }
    }

    fn to_sql(&self) -> String {
        format!(" {} {} {}", self.column, self.operator, self.value)
    }
}

pub struct BoolConstraint {
    column: String,
    operator: String,
    value: bool,
}
impl ConstraintTrait<bool> for BoolConstraint {
    fn new(column: String, operator: String, value: bool) -> Self {
        BoolConstraint { column, operator, value }
    }

    fn to_sql(&self) -> String {
        format!(" {} {} {}", self.column, self.operator, self.value)
    }
}

pub struct DateTimeUtcConstraint {
    column: String,
    operator: String,
    value: DateTime<Utc>,
}
impl ConstraintTrait<DateTime<Utc>> for DateTimeUtcConstraint {
    fn new(column: String, operator: String, value: DateTime<Utc>) -> Self {
        DateTimeUtcConstraint { column, operator, value }
    }

    fn to_sql(&self) -> String {
        // https://docs.rs/sqlx/latest/sqlx/types/chrono/struct.DateTime.html#method.to_rfc3339
        format!(" {} {} {}", self.column, self.operator, self.value.to_rfc3339())
    }
}

pub struct DateTimeLocalConstraint {
    column: String,
    operator: String,
    value: DateTime<Local>,
}
impl ConstraintTrait<DateTime<Local>> for DateTimeLocalConstraint {
    fn new(column: String, operator: String, value: DateTime<Local>) -> Self {
        DateTimeLocalConstraint { column, operator, value }
    }

    fn to_sql(&self) -> String {
        // https://docs.rs/sqlx/latest/sqlx/types/chrono/struct.DateTime.html#method.to_rfc3339
        format!(" {} {} {}", self.column, self.operator, self.value.to_rfc3339())
    }
}

pub enum Constraint {
    String(StringConstraint),
    Bool(BoolConstraint),
    DateTimeUtc(DateTimeUtcConstraint),
    DateTimeLocal(DateTimeLocalConstraint),
}

impl Constraint {
    pub fn to_sql(&self) -> String {
        match self {
            Constraint::String(i) => i.to_sql(),
            Constraint::Bool(i) => i.to_sql(),
            Constraint::DateTimeUtc(i) => i.to_sql(),
            Constraint::DateTimeLocal(i) => i.to_sql(),
            // _ => unimplemented!("Unrecognized Constraint in Constraint.to_sql()"),
        }
    }
}