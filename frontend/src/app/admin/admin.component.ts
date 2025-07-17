import { Component, OnInit } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { ApiService } from '../services/api.service';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-admin',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.css']
})
export class AdminComponent implements OnInit {
  users: any[] = [];
  userColumns: string[] = ['username', 'role', 'actions'];

  constructor(
    private fb: FormBuilder,
    private apiService: ApiService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {
    this.loadUsers();
  }

  loadUsers() {
    this.apiService.get('/admin/users').subscribe(data => {
      this.users = data;
    });
  }

  changeUserRole(userId: string, role: string) {
    this.apiService.put(`/admin/users/${userId}/role`, { role }).subscribe(() => {
      this.snackBar.open('User role updated', 'Close', { duration: 3000 });
      this.loadUsers();
    });
  }
}
