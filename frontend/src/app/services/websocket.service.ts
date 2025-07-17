import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {
  private socket: WebSocket;
  private message$: Subject<any> = new Subject();

  constructor() {
    this.connect('ws://localhost:8000/ws');
  }

  private connect(url: string): void {
    this.socket = new WebSocket(url);

    this.socket.onopen = () => {
      console.log('WebSocket connection established');
    };

    this.socket.onmessage = (event) => {
      this.message$.next(JSON.parse(event.data));
    };

    this.socket.onclose = (event) => {
      console.log('WebSocket connection closed', event);
      // Optional: attempt to reconnect
    };

    this.socket.onerror = (error) => {
      console.error('WebSocket error', error);
    };
  }

  public sendMessage(data: any): void {
    if (this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(data));
    }
  }

  public getMessage(): Observable<any> {
    return this.message$.asObservable();
  }
}
