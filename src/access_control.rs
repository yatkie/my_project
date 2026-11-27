use std::fmt;

/// Ролі користувачів.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

impl Role {
    /// Отримання ролі з налаштувань програми.
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "operator" => Some(Role::Operator),
            "viewer" => Some(Role::Viewer),
            _ => None,
        }
    }
}

/// Дозволи на виконання операцій.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    ViewInfo,
    GenerateKeys,
    CreateMint,
    InitializeMint,
    CreateTokenAccount,
    MintTokens,
    SignTransaction,
}

/// Помилки авторизації.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationError {
    Unauthenticated,
    InsufficientPermissions,
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorizationError::Unauthenticated => {
                write!(f, "Access denied: user is not authenticated.")
            }
            AuthorizationError::InsufficientPermissions => {
                write!(f, "Access denied: insufficient permissions.")
            }
        }
    }
}

impl std::error::Error for AuthorizationError {}

/// Перевірка дозволу відповідно до ролі.
pub fn has_permission(role: Role, permission: Permission) -> bool {
    match role {
        Role::Admin => true,

        Role::Operator => matches!(
            permission,
            Permission::ViewInfo
                | Permission::GenerateKeys
                | Permission::CreateMint
                | Permission::InitializeMint
                | Permission::CreateTokenAccount
        ),

        Role::Viewer => permission == Permission::ViewInfo,
    }
}

/// Перевірка доступу до захищеної операції.
pub fn authorize(role: Option<Role>, permission: Permission) -> Result<(), AuthorizationError> {
    let role = role.ok_or(AuthorizationError::Unauthenticated)?;

    if has_permission(role, permission) {
        Ok(())
    } else {
        Err(AuthorizationError::InsufficientPermissions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_has_full_access() {
        let permissions = [
            Permission::ViewInfo,
            Permission::GenerateKeys,
            Permission::CreateMint,
            Permission::InitializeMint,
            Permission::CreateTokenAccount,
            Permission::MintTokens,
            Permission::SignTransaction,
        ];

        for permission in permissions {
            assert!(has_permission(Role::Admin, permission));
        }
    }

    #[test]
    fn operator_has_limited_access() {
        assert!(has_permission(Role::Operator, Permission::ViewInfo));
        assert!(has_permission(Role::Operator, Permission::GenerateKeys));
        assert!(has_permission(Role::Operator, Permission::CreateMint));
        assert!(has_permission(Role::Operator, Permission::InitializeMint));
        assert!(has_permission(
            Role::Operator,
            Permission::CreateTokenAccount
        ));

        assert!(!has_permission(Role::Operator, Permission::MintTokens));
        assert!(!has_permission(Role::Operator, Permission::SignTransaction));
    }

    #[test]
    fn viewer_can_only_view_information() {
        assert!(has_permission(Role::Viewer, Permission::ViewInfo));

        assert!(!has_permission(Role::Viewer, Permission::GenerateKeys));
        assert!(!has_permission(Role::Viewer, Permission::CreateMint));
        assert!(!has_permission(Role::Viewer, Permission::MintTokens));
        assert!(!has_permission(Role::Viewer, Permission::SignTransaction));
    }

    #[test]
    fn unauthenticated_user_is_denied() {
        assert_eq!(
            authorize(None, Permission::ViewInfo),
            Err(AuthorizationError::Unauthenticated)
        );
    }

    #[test]
    fn unauthorized_operation_is_denied() {
        assert_eq!(
            authorize(Some(Role::Viewer), Permission::MintTokens),
            Err(AuthorizationError::InsufficientPermissions)
        );
    }

    #[test]
    fn authorized_operation_is_allowed() {
        assert_eq!(
            authorize(Some(Role::Admin), Permission::SignTransaction),
            Ok(())
        );
    }
}
