import { Component } from '@angular/core';
import * as pdfjsLib from 'pdfjs-dist';
import { PDFDocument, rgb, StandardFonts } from 'pdf-lib';
import { saveAs } from 'file-saver';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent {

  pdfText = '';
  fileName = '';

  constructor() {
    // Configura o caminho para o worker do PDF.js
    pdfjsLib.GlobalWorkerOptions.workerSrc = '/assets/pdfworker/pdf.worker.min.js';
  }

  handleFileInput(event: any) {
    const file = event.target.files[0];
    console.log(file);

    this.fileName = file.name;

    const fileReader = new FileReader();
    fileReader.onload = async (e) => {
      const arrayBuffer = fileReader.result as ArrayBuffer;

      // Convertendo o ArrayBuffer em Uint8Array
      const uint8Array = new Uint8Array(arrayBuffer);

      const pdf = await pdfjsLib.getDocument({ data: uint8Array }).promise;

      let text = '';
      for (let i = 1; i <= pdf.numPages; i++) {
        const page = await pdf.getPage(i);
        const pageTextContent = await page.getTextContent();
        const pageTextItems = pageTextContent.items.map((item: any) => {
          if (typeof item === 'string') {
            return item;
          } else if (item.hasOwnProperty('str')) {
            return item.str;
          } else {
            return '';
          }
        });
        text += pageTextItems.join(' ');
      }

      console.log('texto contido no pdf: ' + text);
      this.pdfText = text;
      // Mostra o texto no HTML
      const textContainer = document.getElementById('text-container');
      textContainer!.innerText = text;
    };

    fileReader.readAsArrayBuffer(file);
  }

  
  
}