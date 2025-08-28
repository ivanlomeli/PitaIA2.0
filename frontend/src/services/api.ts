import axios from 'axios';
import { AuthResponse, ApiResponse, LoginData, RegisterData, User } from '../types';

const API_BASE_URL = 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Interceptor para agregar token automáticamente
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('pitaia_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export class AuthService {
  static async register(data: RegisterData): Promise<AuthResponse> {
    const response = await api.post<ApiResponse<AuthResponse>>('/auth/register', data);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }

  static async login(data: LoginData): Promise<AuthResponse> {
    const response = await api.post<ApiResponse<AuthResponse>>('/auth/login', data);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }

  static async getUserProfile(username: string): Promise<User> {
    const response = await api.get<ApiResponse<User>>(`/users/${username}`);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }
}

export interface Post {
  id: string;
  user_id: string;
  username: string;
  display_name: string | null;
  avatar_url: string | null;
  content: string;
  image_url: string | null;
  likes_count: number;
  comments_count: number;
  created_at: string;
  is_liked?: boolean;
}

export interface CreatePostData {
  content: string;
  image_url?: string;
}

export class PostService {
  static async createPost(data: CreatePostData): Promise<Post> {
    const response = await api.post<ApiResponse<Post>>('/posts', data);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }

  static async getFeed(limit = 20, offset = 0): Promise<Post[]> {
    const response = await api.get<ApiResponse<Post[]>>(`/posts?limit=${limit}&offset=${offset}`);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }

  static async toggleLike(postId: string): Promise<boolean> {
    const response = await api.post<ApiResponse<boolean>>(`/posts/${postId}/like`);
    if (response.data.success && response.data.data !== null) {
      return response.data.data;
    }
    throw new Error(response.data.message);
  }
}
