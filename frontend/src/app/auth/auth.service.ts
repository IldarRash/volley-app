import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { Observable, BehaviorSubject } from 'rxjs';
import { tap } from 'rxjs/operators';
import { jwtDecode } from 'jwt-decode';
import { User } from '../models/user.model';

interface JwtPayload {
  sub: string;
  username: string;
  role: string;
  exp: number;
  iat: number;
}

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private apiUrl = 'http://localhost:8080/auth';
  private tokenKey = 'auth_token';
  private currentUserSubject = new BehaviorSubject<User | null>(null);
  public currentUser = this.currentUserSubject.asObservable();

  constructor(private http: HttpClient, private router: Router) {
    // Check if user is already logged in on service initialization
    this.checkCurrentUser();
  }

  private checkCurrentUser(): void {
    const token = localStorage.getItem('token');
    if (token && !this.isTokenExpired(token)) {
      const decoded = jwtDecode<JwtPayload>(token);
      const userId = localStorage.getItem('userId');
      if (userId) {
        // Fetch user details from API
        this.http.get<User>(`http://localhost:8080/api/users/${userId}`).subscribe(
          user => this.currentUserSubject.next(user),
          error => {
            console.error('Failed to fetch user details', error);
            this.currentUserSubject.next(null);
          }
        );
      }
    } else {
      this.currentUserSubject.next(null);
    }
  }

  register(user: any): Observable<any> {
    return this.http.post(`${this.apiUrl}/register`, user);
  }

  login(credentials: any): Observable<any> {
    return this.http.post<any>(`${this.apiUrl}/login`, credentials).pipe(
      tap((response: any) => {
        localStorage.setItem('token', response.token);
        localStorage.setItem('userId', response.user_id);
        this.checkCurrentUser(); // Update current user after login
      })
    );
  }

  logout(): void {
    localStorage.removeItem('token');
    localStorage.removeItem('userId');
    this.currentUserSubject.next(null);
  }

  getUserId(): string | null {
    return localStorage.getItem('userId');
  }

  isLoggedIn(): boolean {
    return !!localStorage.getItem('token');
  }

  getToken(): string | null {
    return localStorage.getItem(this.tokenKey);
  }

  private isTokenExpired(token: string): boolean {
    try {
      const decoded: any = jwtDecode(token);
      return decoded.exp < Date.now() / 1000;
    } catch (error) {
      return true;
    }
  }

  isAdmin(): boolean {
    const token = this.getToken();
    if (!token || this.isTokenExpired(token)) {
      return false;
    }
    
    try {
      const decoded: any = jwtDecode(token);
      return decoded.role === 'Admin';
    } catch (error) {
      return false;
    }
  }
}
