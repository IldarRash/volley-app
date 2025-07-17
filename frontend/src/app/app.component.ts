import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from './auth/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'volley-app';
  isLoggedIn = false;
  isAdmin = false;
  isMobileMenuOpen = false;
  isUserMenuOpen = false;
  userName: string | null = null;
  userEmail: string | null = null;
  userAvatar: string | null = null;

  constructor(
    private authService: AuthService,
    private router: Router
  ) {}

  ngOnInit() {
    this.authService.currentUser.subscribe(user => {
      this.isLoggedIn = !!user;
      if (user) {
        this.isAdmin = user.role === 'Admin';
        this.userName = user.username;
        this.userEmail = `${user.username}@volleyapp.com`;
        this.userAvatar = user.image_url || null;
      } else {
        this.isAdmin = false;
        this.userName = null;
        this.userEmail = null;
        this.userAvatar = null;
      }
    });

    // Close menus on route change
    this.router.events.subscribe(() => {
      this.closeMobileMenu();
      this.closeUserMenu();
    });
  }

  toggleMobileMenu() {
    this.isMobileMenuOpen = !this.isMobileMenuOpen;
    if (this.isMobileMenuOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
  }

  closeMobileMenu() {
    this.isMobileMenuOpen = false;
    document.body.style.overflow = '';
  }

  toggleUserMenu() {
    this.isUserMenuOpen = !this.isUserMenuOpen;
  }

  closeUserMenu() {
    this.isUserMenuOpen = false;
  }

  logout() {
    this.authService.logout();
    this.closeUserMenu();
    this.router.navigate(['/login']);
  }
} 