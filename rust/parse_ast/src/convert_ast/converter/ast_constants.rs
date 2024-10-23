// This file is generated by scripts/generate-rust-constants.js.
// Do not edit this file directly.

pub const TYPE_PANIC_ERROR: [u8; 4] = 0u32.to_ne_bytes();
pub const TYPE_PARSE_ERROR: [u8; 4] = 1u32.to_ne_bytes();
pub const TYPE_ARRAY_EXPRESSION: [u8; 4] = 2u32.to_ne_bytes();
pub const TYPE_ARRAY_PATTERN: [u8; 4] = 3u32.to_ne_bytes();
pub const TYPE_ARROW_FUNCTION_EXPRESSION: [u8; 4] = 4u32.to_ne_bytes();
pub const TYPE_ASSIGNMENT_PATTERN: [u8; 4] = 6u32.to_ne_bytes();
pub const TYPE_BINARY_EXPRESSION: [u8; 4] = 8u32.to_ne_bytes();
pub const TYPE_BLOCK_STATEMENT: [u8; 4] = 9u32.to_ne_bytes();
pub const TYPE_CALL_EXPRESSION: [u8; 4] = 11u32.to_ne_bytes();
pub const TYPE_CATCH_CLAUSE: [u8; 4] = 12u32.to_ne_bytes();
pub const TYPE_CHAIN_EXPRESSION: [u8; 4] = 13u32.to_ne_bytes();
pub const TYPE_CLASS_BODY: [u8; 4] = 14u32.to_ne_bytes();
pub const TYPE_CLASS_DECLARATION: [u8; 4] = 15u32.to_ne_bytes();
pub const TYPE_CLASS_EXPRESSION: [u8; 4] = 16u32.to_ne_bytes();
pub const TYPE_EXPORT_ALL_DECLARATION: [u8; 4] = 24u32.to_ne_bytes();
pub const TYPE_EXPORT_DEFAULT_DECLARATION: [u8; 4] = 25u32.to_ne_bytes();
pub const TYPE_EXPORT_NAMED_DECLARATION: [u8; 4] = 26u32.to_ne_bytes();
pub const TYPE_FUNCTION_DECLARATION: [u8; 4] = 32u32.to_ne_bytes();
pub const TYPE_FUNCTION_EXPRESSION: [u8; 4] = 33u32.to_ne_bytes();
pub const TYPE_IDENTIFIER: [u8; 4] = 34u32.to_ne_bytes();
pub const TYPE_IMPORT_ATTRIBUTE: [u8; 4] = 36u32.to_ne_bytes();
pub const TYPE_IMPORT_DECLARATION: [u8; 4] = 37u32.to_ne_bytes();
pub const TYPE_IMPORT_EXPRESSION: [u8; 4] = 39u32.to_ne_bytes();
pub const TYPE_JSX_EMPTY_EXPRESSION: [u8; 4] = 46u32.to_ne_bytes();
pub const TYPE_JSX_EXPRESSION_CONTAINER: [u8; 4] = 47u32.to_ne_bytes();
pub const TYPE_JSX_MEMBER_EXPRESSION: [u8; 4] = 50u32.to_ne_bytes();
pub const TYPE_JSX_NAMESPACED_NAME: [u8; 4] = 51u32.to_ne_bytes();
pub const TYPE_JSX_OPENING_ELEMENT: [u8; 4] = 52u32.to_ne_bytes();
pub const TYPE_JSX_SPREAD_ATTRIBUTE: [u8; 4] = 54u32.to_ne_bytes();
pub const TYPE_LOGICAL_EXPRESSION: [u8; 4] = 64u32.to_ne_bytes();
pub const TYPE_MEMBER_EXPRESSION: [u8; 4] = 65u32.to_ne_bytes();
pub const TYPE_META_PROPERTY: [u8; 4] = 66u32.to_ne_bytes();
pub const TYPE_METHOD_DEFINITION: [u8; 4] = 67u32.to_ne_bytes();
pub const TYPE_NEW_EXPRESSION: [u8; 4] = 68u32.to_ne_bytes();
pub const TYPE_PROGRAM: [u8; 4] = 72u32.to_ne_bytes();
pub const TYPE_PROPERTY: [u8; 4] = 73u32.to_ne_bytes();
pub const TYPE_PROPERTY_DEFINITION: [u8; 4] = 74u32.to_ne_bytes();
pub const TYPE_REST_ELEMENT: [u8; 4] = 75u32.to_ne_bytes();
pub const TYPE_SPREAD_ELEMENT: [u8; 4] = 78u32.to_ne_bytes();
pub const TYPE_TEMPLATE_LITERAL: [u8; 4] = 85u32.to_ne_bytes();
pub const TYPE_TRY_STATEMENT: [u8; 4] = 88u32.to_ne_bytes();
pub const TYPE_VARIABLE_DECLARATION: [u8; 4] = 91u32.to_ne_bytes();
pub const TYPE_VARIABLE_DECLARATOR: [u8; 4] = 92u32.to_ne_bytes();

pub const PANIC_ERROR_RESERVED_BYTES: usize = 8;
pub const PANIC_ERROR_MESSAGE_OFFSET: usize = 4;

pub const PARSE_ERROR_RESERVED_BYTES: usize = 8;
pub const PARSE_ERROR_MESSAGE_OFFSET: usize = 4;

pub const ARRAY_EXPRESSION_RESERVED_BYTES: usize = 8;
pub const ARRAY_EXPRESSION_ELEMENTS_OFFSET: usize = 4;

pub const ARRAY_PATTERN_RESERVED_BYTES: usize = 8;
pub const ARRAY_PATTERN_ELEMENTS_OFFSET: usize = 4;

pub const ARROW_FUNCTION_EXPRESSION_RESERVED_BYTES: usize = 20;
pub const ARROW_FUNCTION_EXPRESSION_ANNOTATIONS_OFFSET: usize = 8;
pub const ARROW_FUNCTION_EXPRESSION_PARAMS_OFFSET: usize = 12;
pub const ARROW_FUNCTION_EXPRESSION_BODY_OFFSET: usize = 16;

pub const ASSIGNMENT_PATTERN_RESERVED_BYTES: usize = 12;
pub const ASSIGNMENT_PATTERN_LEFT_OFFSET: usize = 4;
pub const ASSIGNMENT_PATTERN_RIGHT_OFFSET: usize = 8;

pub const BINARY_EXPRESSION_RESERVED_BYTES: usize = 16;
pub const BINARY_EXPRESSION_OPERATOR_OFFSET: usize = 4;
pub const BINARY_EXPRESSION_LEFT_OFFSET: usize = 8;
pub const BINARY_EXPRESSION_RIGHT_OFFSET: usize = 12;

pub const BLOCK_STATEMENT_RESERVED_BYTES: usize = 8;
pub const BLOCK_STATEMENT_BODY_OFFSET: usize = 4;

pub const CALL_EXPRESSION_RESERVED_BYTES: usize = 20;
pub const CALL_EXPRESSION_ANNOTATIONS_OFFSET: usize = 8;
pub const CALL_EXPRESSION_CALLEE_OFFSET: usize = 12;
pub const CALL_EXPRESSION_ARGUMENTS_OFFSET: usize = 16;

pub const CATCH_CLAUSE_RESERVED_BYTES: usize = 12;
pub const CATCH_CLAUSE_PARAM_OFFSET: usize = 4;
pub const CATCH_CLAUSE_BODY_OFFSET: usize = 8;

pub const CHAIN_EXPRESSION_RESERVED_BYTES: usize = 8;
pub const CHAIN_EXPRESSION_EXPRESSION_OFFSET: usize = 4;

pub const CLASS_BODY_RESERVED_BYTES: usize = 8;
pub const CLASS_BODY_BODY_OFFSET: usize = 4;

pub const CLASS_DECLARATION_RESERVED_BYTES: usize = 20;
pub const CLASS_DECLARATION_DECORATORS_OFFSET: usize = 4;
pub const CLASS_DECLARATION_ID_OFFSET: usize = 8;
pub const CLASS_DECLARATION_SUPER_CLASS_OFFSET: usize = 12;
pub const CLASS_DECLARATION_BODY_OFFSET: usize = 16;

pub const EXPORT_ALL_DECLARATION_RESERVED_BYTES: usize = 16;
pub const EXPORT_ALL_DECLARATION_EXPORTED_OFFSET: usize = 4;
pub const EXPORT_ALL_DECLARATION_SOURCE_OFFSET: usize = 8;
pub const EXPORT_ALL_DECLARATION_ATTRIBUTES_OFFSET: usize = 12;

pub const EXPORT_DEFAULT_DECLARATION_RESERVED_BYTES: usize = 8;
pub const EXPORT_DEFAULT_DECLARATION_DECLARATION_OFFSET: usize = 4;

pub const EXPORT_NAMED_DECLARATION_RESERVED_BYTES: usize = 20;
pub const EXPORT_NAMED_DECLARATION_SPECIFIERS_OFFSET: usize = 4;
pub const EXPORT_NAMED_DECLARATION_SOURCE_OFFSET: usize = 8;
pub const EXPORT_NAMED_DECLARATION_ATTRIBUTES_OFFSET: usize = 12;
pub const EXPORT_NAMED_DECLARATION_DECLARATION_OFFSET: usize = 16;

pub const FUNCTION_DECLARATION_RESERVED_BYTES: usize = 24;
pub const FUNCTION_DECLARATION_ANNOTATIONS_OFFSET: usize = 8;
pub const FUNCTION_DECLARATION_ID_OFFSET: usize = 12;
pub const FUNCTION_DECLARATION_PARAMS_OFFSET: usize = 16;
pub const FUNCTION_DECLARATION_BODY_OFFSET: usize = 20;

pub const IDENTIFIER_RESERVED_BYTES: usize = 8;
pub const IDENTIFIER_NAME_OFFSET: usize = 4;

pub const IMPORT_ATTRIBUTE_RESERVED_BYTES: usize = 12;
pub const IMPORT_ATTRIBUTE_KEY_OFFSET: usize = 4;
pub const IMPORT_ATTRIBUTE_VALUE_OFFSET: usize = 8;

pub const IMPORT_DECLARATION_RESERVED_BYTES: usize = 16;
pub const IMPORT_DECLARATION_SPECIFIERS_OFFSET: usize = 4;
pub const IMPORT_DECLARATION_SOURCE_OFFSET: usize = 8;
pub const IMPORT_DECLARATION_ATTRIBUTES_OFFSET: usize = 12;

pub const IMPORT_EXPRESSION_RESERVED_BYTES: usize = 12;
pub const IMPORT_EXPRESSION_SOURCE_OFFSET: usize = 4;
pub const IMPORT_EXPRESSION_OPTIONS_OFFSET: usize = 8;

pub const JSX_EMPTY_EXPRESSION_RESERVED_BYTES: usize = 4;

pub const JSX_EXPRESSION_CONTAINER_RESERVED_BYTES: usize = 8;
pub const JSX_EXPRESSION_CONTAINER_EXPRESSION_OFFSET: usize = 4;

pub const JSX_MEMBER_EXPRESSION_RESERVED_BYTES: usize = 12;
pub const JSX_MEMBER_EXPRESSION_OBJECT_OFFSET: usize = 4;
pub const JSX_MEMBER_EXPRESSION_PROPERTY_OFFSET: usize = 8;

pub const JSX_NAMESPACED_NAME_RESERVED_BYTES: usize = 12;
pub const JSX_NAMESPACED_NAME_NAMESPACE_OFFSET: usize = 4;
pub const JSX_NAMESPACED_NAME_NAME_OFFSET: usize = 8;

pub const JSX_OPENING_ELEMENT_RESERVED_BYTES: usize = 16;
pub const JSX_OPENING_ELEMENT_NAME_OFFSET: usize = 8;
pub const JSX_OPENING_ELEMENT_ATTRIBUTES_OFFSET: usize = 12;

pub const JSX_SPREAD_ATTRIBUTE_RESERVED_BYTES: usize = 8;
pub const JSX_SPREAD_ATTRIBUTE_ARGUMENT_OFFSET: usize = 4;

pub const MEMBER_EXPRESSION_RESERVED_BYTES: usize = 16;
pub const MEMBER_EXPRESSION_OBJECT_OFFSET: usize = 8;
pub const MEMBER_EXPRESSION_PROPERTY_OFFSET: usize = 12;

pub const META_PROPERTY_RESERVED_BYTES: usize = 12;
pub const META_PROPERTY_META_OFFSET: usize = 4;
pub const META_PROPERTY_PROPERTY_OFFSET: usize = 8;

pub const METHOD_DEFINITION_RESERVED_BYTES: usize = 24;
pub const METHOD_DEFINITION_DECORATORS_OFFSET: usize = 8;
pub const METHOD_DEFINITION_KEY_OFFSET: usize = 12;
pub const METHOD_DEFINITION_VALUE_OFFSET: usize = 16;
pub const METHOD_DEFINITION_KIND_OFFSET: usize = 20;

pub const NEW_EXPRESSION_RESERVED_BYTES: usize = 16;
pub const NEW_EXPRESSION_ANNOTATIONS_OFFSET: usize = 4;
pub const NEW_EXPRESSION_CALLEE_OFFSET: usize = 8;
pub const NEW_EXPRESSION_ARGUMENTS_OFFSET: usize = 12;

pub const PROGRAM_RESERVED_BYTES: usize = 12;
pub const PROGRAM_BODY_OFFSET: usize = 4;
pub const PROGRAM_INVALID_ANNOTATIONS_OFFSET: usize = 8;

pub const PROPERTY_RESERVED_BYTES: usize = 20;
pub const PROPERTY_KEY_OFFSET: usize = 8;
pub const PROPERTY_VALUE_OFFSET: usize = 12;
pub const PROPERTY_KIND_OFFSET: usize = 16;

pub const PROPERTY_DEFINITION_RESERVED_BYTES: usize = 20;
pub const PROPERTY_DEFINITION_DECORATORS_OFFSET: usize = 8;
pub const PROPERTY_DEFINITION_KEY_OFFSET: usize = 12;
pub const PROPERTY_DEFINITION_VALUE_OFFSET: usize = 16;

pub const REST_ELEMENT_RESERVED_BYTES: usize = 8;
pub const REST_ELEMENT_ARGUMENT_OFFSET: usize = 4;

pub const SPREAD_ELEMENT_RESERVED_BYTES: usize = 8;
pub const SPREAD_ELEMENT_ARGUMENT_OFFSET: usize = 4;

pub const TEMPLATE_LITERAL_RESERVED_BYTES: usize = 12;
pub const TEMPLATE_LITERAL_QUASIS_OFFSET: usize = 4;
pub const TEMPLATE_LITERAL_EXPRESSIONS_OFFSET: usize = 8;

pub const TRY_STATEMENT_RESERVED_BYTES: usize = 16;
pub const TRY_STATEMENT_BLOCK_OFFSET: usize = 4;
pub const TRY_STATEMENT_HANDLER_OFFSET: usize = 8;
pub const TRY_STATEMENT_FINALIZER_OFFSET: usize = 12;

pub const VARIABLE_DECLARATION_RESERVED_BYTES: usize = 12;
pub const VARIABLE_DECLARATION_KIND_OFFSET: usize = 4;
pub const VARIABLE_DECLARATION_DECLARATIONS_OFFSET: usize = 8;

pub const VARIABLE_DECLARATOR_RESERVED_BYTES: usize = 12;
pub const VARIABLE_DECLARATOR_ID_OFFSET: usize = 4;
pub const VARIABLE_DECLARATOR_INIT_OFFSET: usize = 8;