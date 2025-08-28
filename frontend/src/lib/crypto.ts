export class PQCrypto {
  private keyPair: { publicKey: Uint8Array; secretKey: Uint8Array } | null = null;
  private serverPublicKey: Uint8Array | null = null;

  async generateKeyPair(): Promise<{ publicKey: Uint8Array; secretKey: Uint8Array }> {
    if (typeof window === 'undefined') {
      const crypto = await import('crypto');
      const publicKey = crypto.randomBytes(32);
      const secretKey = crypto.randomBytes(32);
      
      this.keyPair = { 
        publicKey: new Uint8Array(publicKey), 
        secretKey: new Uint8Array(secretKey) 
      };
    } else {
      const publicKey = new Uint8Array(32);
      const secretKey = new Uint8Array(32);
      crypto.getRandomValues(publicKey);
      crypto.getRandomValues(secretKey);
      
      this.keyPair = { publicKey, secretKey };
    }
    
    return this.keyPair;
  }

  async exchangeKeys(serverUrl: string): Promise<void> {
    if (!this.keyPair) {
      await this.generateKeyPair();
    }

    try {
      const response = await fetch(`${serverUrl}/api/crypto/exchange`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          publicKey: Array.from(this.keyPair!.publicKey),
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to exchange keys');
      }

      const { serverPublicKey } = await response.json();
      this.serverPublicKey = new Uint8Array(serverPublicKey);
    } catch (error) {
      console.error('Key exchange failed:', error);
      throw error;
    }
  }

  async encrypt(data: string): Promise<string> {
    if (!this.serverPublicKey) {
      throw new Error('Server public key not available. Call exchangeKeys() first.');
    }

    if (typeof window === 'undefined') {
      const crypto = await import('crypto');
      const algorithm = 'aes-256-cbc';
      const key = this.serverPublicKey.slice(0, 32);
      const iv = crypto.randomBytes(16);
      
      const cipher = crypto.createCipher(algorithm, Buffer.from(key));
      cipher.setAutoPadding(true);
      let encrypted = cipher.update(data, 'utf8', 'hex');
      encrypted += cipher.final('hex');
      
      return Buffer.concat([iv, Buffer.from(encrypted, 'hex')]).toString('base64');
    } else {
      const encoder = new TextEncoder();
      const dataBuffer = encoder.encode(data);
      const key = await crypto.subtle.importKey(
        'raw',
        this.serverPublicKey.slice(0, 32),
        { name: 'AES-GCM' },
        false,
        ['encrypt']
      );
      
      const iv = new Uint8Array(12);
      crypto.getRandomValues(iv);
      
      const encrypted = await crypto.subtle.encrypt(
        { name: 'AES-GCM', iv },
        key,
        dataBuffer
      );
      
      const result = new Uint8Array(iv.length + encrypted.byteLength);
      result.set(iv, 0);
      result.set(new Uint8Array(encrypted), iv.length);
      
      return btoa(String.fromCharCode.apply(null, Array.from(result)));
    }
  }

  async decrypt(encryptedData: string): Promise<string> {
    if (!this.keyPair) {
      throw new Error('Key pair not available');
    }

    if (typeof window === 'undefined') {
      const crypto = await import('crypto');
      const algorithm = 'aes-256-cbc';
      const key = this.keyPair.secretKey.slice(0, 32);
      const buffer = Buffer.from(encryptedData, 'base64');
      const iv = buffer.slice(0, 16);
      const encrypted = buffer.slice(16);
      
      const decipher = crypto.createDecipher(algorithm, Buffer.from(key));
      decipher.setAutoPadding(true);
      let decrypted = decipher.update(encrypted, undefined, 'utf8');
      decrypted += decipher.final('utf8');
      
      return decrypted;
    } else {
      const encryptedBuffer = Uint8Array.from(atob(encryptedData), c => c.charCodeAt(0));
      const iv = encryptedBuffer.slice(0, 12);
      const data = encryptedBuffer.slice(12);
      
      const key = await crypto.subtle.importKey(
        'raw',
        this.keyPair.secretKey.slice(0, 32),
        { name: 'AES-GCM' },
        false,
        ['decrypt']
      );
      
      const decrypted = await crypto.subtle.decrypt(
        { name: 'AES-GCM', iv },
        key,
        data
      );
      
      const decoder = new TextDecoder();
      return decoder.decode(decrypted);
    }
  }

  getPublicKey(): Uint8Array | null {
    return this.keyPair?.publicKey || null;
  }
}