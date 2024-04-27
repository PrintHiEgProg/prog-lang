#[derive(Debug, Clone)]
pub struct Program {
	pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum Statement {
	VariableDefine {
		name: String,
		value: Option<expressions::Expression>
	},

	VariableAssign {
		name: String,
		value: expressions::Expression
	}
}

pub mod expressions {
	#[derive(Debug, Clone)]
	pub enum Expression {
		Unary(Unary),
		Binary(Binary),
		Term(Term)
	}

	#[derive(Debug, Clone)]
	pub struct Unary {
		pub operator: operators::UnaryOperator,
		pub operand: Term
	}

	#[derive(Debug, Clone)]
	pub struct Binary {
		pub lhs: Term,
		pub operator: operators::BinaryOperator,
		pub rhs: Term
	}

	#[derive(Debug, Clone)]
	pub enum Term {
		Literal(Literal),
		Identifier(String),
		Expression(Box<Expression>)
	}

	#[derive(Debug, Clone)]
	pub enum Literal {
		Boolean(bool),
		String(String),
		Number(f64)
	}

	pub mod operators {
		#[derive(Debug, Clone, Copy)]
		pub enum BinaryOperator {
			Plus,
			Minus,
			Divide,
			Multiply,
			Modulo,
			EqEq,
			NotEq,
			And,
			Or,
			Gt,
			Lt,
			Gte,
			Lte
		}

		#[derive(Debug, Clone, Copy)]
		pub enum UnaryOperator {
			Minus,
			Not
		}
	}
}

// implementations
impl From<expressions::Literal> for expressions::Term {
	fn from(value: expressions::Literal) -> Self {
		Self::Literal(value)
	}
}

impl From<expressions::Expression> for expressions::Term {
	fn from(value: expressions::Expression) -> Self {
		Self::Expression(
			Box::new(value)
		)
	}
}

impl From<expressions::Unary> for expressions::Term {
	fn from(value: expressions::Unary) -> Self {
		Self::from(
			expressions::Expression::Unary(value)
		)
	}
}

impl From<expressions::Binary> for expressions::Term {
	fn from(value: expressions::Binary) -> Self {
		Self::from(
			expressions::Expression::Binary(value)
		)
	}
}

impl Into<expressions::Expression> for expressions::Term {
	fn into(self) -> expressions::Expression {
		match self {
			Self::Expression(expression) => *expression,
			_ => expressions::Expression::Term(self)
		}
	}
}

impl expressions::operators::BinaryOperator {
	pub fn get_precedence(&self) -> u8 {
		match self {
			Self::EqEq | Self::NotEq | Self::And | Self::Or | Self::Gt | Self::Lt | Self::Gte | Self::Lte => 1,
			Self::Plus | Self::Minus => 2,
			Self::Multiply | Self::Divide | Self::Modulo => 3,
		}
	}
}

impl TryFrom<String> for expressions::operators::BinaryOperator {
	type Error = String;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match &value[..] {
			"+" => Ok(Self::Plus),
			"-" => Ok(Self::Minus),
			"/" => Ok(Self::Divide),
			"*" => Ok(Self::Multiply),
			"%" => Ok(Self::Modulo),
			"==" => Ok(Self::EqEq),
			"!=" => Ok(Self::NotEq),
			"&&" => Ok(Self::And),
			"||" => Ok(Self::Or),
			">" => Ok(Self::Gt),
			"<" => Ok(Self::Lt),
			">=" => Ok(Self::Gte),
			"<=" => Ok(Self::Lte),

			op => Err(format!("Invalid binary operator '{op}'"))
		}
	}
}

impl TryFrom<String> for expressions::operators::UnaryOperator {
	type Error = String;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match &value[..] {
			"-" => Ok(Self::Minus),
			"!" => Ok(Self::Not),

			op => Err(format!("Invalid unary operator '{op}'"))
		}
	}
}