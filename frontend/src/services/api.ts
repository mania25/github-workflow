import { Todo, CreateTodoRequest, UpdateTodoRequest } from '@/types/todo';
import { PQCrypto } from '@/lib/crypto';

class ApiService {
  private baseUrl: string;
  private crypto: PQCrypto;

  constructor() {
    this.baseUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    this.crypto = new PQCrypto();
  }

  async initializeCrypto() {
    await this.crypto.generateKeyPair();
    await this.crypto.exchangeKeys(this.baseUrl);
  }

  private async makeSecureRequest(endpoint: string, options: RequestInit = {}) {
    const url = `${this.baseUrl}${endpoint}`;
    
    if (options.body && typeof options.body === 'string') {
      options.body = await this.crypto.encrypt(options.body);
    }

    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response;
  }

  async getTodos(): Promise<Todo[]> {
    const response = await this.makeSecureRequest('/api/todos');
    return response.json();
  }

  async createTodo(todo: CreateTodoRequest): Promise<Todo> {
    const response = await this.makeSecureRequest('/api/todos', {
      method: 'POST',
      body: JSON.stringify(todo),
    });
    return response.json();
  }

  async updateTodo(id: string, todo: UpdateTodoRequest): Promise<Todo> {
    const response = await this.makeSecureRequest(`/api/todos/${id}`, {
      method: 'PUT',
      body: JSON.stringify(todo),
    });
    return response.json();
  }

  async deleteTodo(id: string): Promise<void> {
    await this.makeSecureRequest(`/api/todos/${id}`, {
      method: 'DELETE',
    });
  }
}

export const apiService = new ApiService();