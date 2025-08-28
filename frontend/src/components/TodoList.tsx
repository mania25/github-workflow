'use client';

import { useState, useEffect } from 'react';
import { Todo } from '@/types/todo';
import { apiService } from '@/services/api';

export default function TodoList() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTodo, setNewTodo] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadTodos();
  }, []);

  const loadTodos = async () => {
    try {
      await apiService.initializeCrypto();
      const fetchedTodos = await apiService.getTodos();
      setTodos(fetchedTodos);
    } catch (error) {
      console.error('Failed to load todos:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddTodo = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTodo.trim()) return;

    try {
      const todo = await apiService.createTodo({ title: newTodo });
      setTodos([...todos, todo]);
      setNewTodo('');
    } catch (error) {
      console.error('Failed to create todo:', error);
    }
  };

  const handleToggleTodo = async (id: string, completed: boolean) => {
    try {
      const updatedTodo = await apiService.updateTodo(id, { completed });
      setTodos(todos.map(todo => todo.id === id ? updatedTodo : todo));
    } catch (error) {
      console.error('Failed to update todo:', error);
    }
  };

  const handleDeleteTodo = async (id: string) => {
    try {
      await apiService.deleteTodo(id);
      setTodos(todos.filter(todo => todo.id !== id));
    } catch (error) {
      console.error('Failed to delete todo:', error);
    }
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading...</div>;
  }

  return (
    <div className="max-w-md mx-auto mt-8 p-6 bg-white rounded-lg shadow-md">
      <h1 className="text-2xl font-bold mb-4 text-center">Todo List</h1>
      
      <form onSubmit={handleAddTodo} className="mb-4">
        <div className="flex gap-2">
          <input
            type="text"
            value={newTodo}
            onChange={(e) => setNewTodo(e.target.value)}
            placeholder="Add a new todo..."
            className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            Add
          </button>
        </div>
      </form>

      <div className="space-y-2">
        {todos.map((todo) => (
          <div key={todo.id} className="flex items-center gap-2 p-2 border border-gray-200 rounded-md">
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={(e) => handleToggleTodo(todo.id, e.target.checked)}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <span className={`flex-1 ${todo.completed ? 'line-through text-gray-500' : ''}`}>
              {todo.title}
            </span>
            <button
              onClick={() => handleDeleteTodo(todo.id)}
              className="px-2 py-1 bg-red-500 text-white text-sm rounded hover:bg-red-600"
            >
              Delete
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}