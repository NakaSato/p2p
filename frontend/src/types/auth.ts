import { z } from 'zod';
import { ReactNode, ComponentType } from 'react';

// Enums
export enum UserType {
    STUDENT = 'student',
    FACULTY = 'faculty',
    STAFF = 'staff',
    ADMIN = 'admin'
}

export enum Role {
    USER = 'user',
    MODERATOR = 'moderator',
    ADMIN = 'admin',
    SUPER_ADMIN = 'super_admin'
}

// Core Interfaces
export interface User {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
    userType: UserType;
    role: Role;
    isVerified: boolean;
    createdAt: string;
    updatedAt: string;
}

export interface AuthTokens {
    accessToken: string;
    refreshToken: string;
    expiresAt: number;
}

export interface AuthState {
    user: User | null;
    tokens: AuthTokens | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    error: string | null;
}

// Form Data Types
export interface LoginFormData {
    email: string;
    password: string;
    rememberMe: boolean;
}

export interface RegisterFormData {
    email: string;
    password: string;
    confirmPassword: string;
    firstName: string;
    lastName: string;
    userType: UserType;
}

export interface PasswordResetFormData {
    email: string;
}

export interface NewPasswordFormData {
    password: string;
    confirmPassword: string;
}

// API Response Types
export interface AuthResponse {
    user: User;
    tokens: AuthTokens;
    message: string;
}

export interface ApiError {
    message: string;
    code: string;
    details?: Record<string, string[]>;
}

// Zod Validation Schemas
export const UserSchema = z.object({
    id: z.string().uuid(),
    email: z.string().email(),
    firstName: z.string().min(1).max(50),
    lastName: z.string().min(1).max(50),
    userType: z.nativeEnum(UserType),
    role: z.nativeEnum(Role),
    isVerified: z.boolean(),
    createdAt: z.string().datetime(),
    updatedAt: z.string().datetime(),
});

export const LoginFormSchema = z.object({
    email: z.string().email('Please enter a valid email address'),
    password: z.string().min(1, 'Password is required'),
    rememberMe: z.boolean().default(false),
});

export const RegisterFormSchema = z.object({
    email: z.string().email('Please enter a valid email address'),
    password: z.string()
        .min(8, 'Password must be at least 8 characters')
        .regex(
            /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/,
            'Password must contain uppercase, lowercase, number, and special character'
        ),
    confirmPassword: z.string(),
    firstName: z.string().min(1, 'First name is required').max(50),
    lastName: z.string().min(1, 'Last name is required').max(50),
    userType: z.nativeEnum(UserType),
}).refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
});

export const PasswordResetFormSchema = z.object({
    email: z.string().email('Please enter a valid email address'),
});

export const NewPasswordFormSchema = z.object({
    password: z.string()
        .min(8, 'Password must be at least 8 characters')
        .regex(
            /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/,
            'Password must contain uppercase, lowercase, number, and special character'
        ),
    confirmPassword: z.string(),
}).refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
});

// Auth Error Types
export enum AuthErrorCode {
    INVALID_CREDENTIALS = 'INVALID_CREDENTIALS',
    EMAIL_NOT_VERIFIED = 'EMAIL_NOT_VERIFIED',
    TOKEN_EXPIRED = 'TOKEN_EXPIRED',
    TOKEN_INVALID = 'TOKEN_INVALID',
    USER_NOT_FOUND = 'USER_NOT_FOUND',
    EMAIL_ALREADY_EXISTS = 'EMAIL_ALREADY_EXISTS',
    WEAK_PASSWORD = 'WEAK_PASSWORD',
    NETWORK_ERROR = 'NETWORK_ERROR',
    SERVER_ERROR = 'SERVER_ERROR',
    PERMISSION_DENIED = 'PERMISSION_DENIED'
}

export class AuthError extends Error {
    constructor(
        public code: AuthErrorCode,
        message: string,
        public details?: Record<string, string[]>
    ) {
        super(message);
        this.name = 'AuthError';
    }
}

// Context Types
export interface AuthContextType {
    // State
    user: User | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    error: string | null;

    // Actions
    login: (credentials: LoginFormData) => Promise<void>;
    register: (userData: RegisterFormData) => Promise<void>;
    logout: () => void;
    resetPassword: (email: string) => Promise<void>;
    setNewPassword: (token: string, password: string) => Promise<void>;
    verifyEmail: (token: string) => Promise<void>;
    resendVerification: () => Promise<void>;
    refreshToken: () => Promise<void>;
    clearError: () => void;
}

// Protected Route Types
export interface ProtectedRouteProps {
    children: ReactNode;
    requiredRole?: Role;
    requireVerification?: boolean;
    fallback?: ComponentType;
}