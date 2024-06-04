import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { Router } from '@angular/router';


@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {  

  constructor(private router: Router) {}

  login() {
    // Aqui você pode adicionar a lógica para fazer a autenticação
    // Se a autenticação for bem-sucedida, navegue para a rota /home
    this.router.navigate(['/home']);
  }
  
}
