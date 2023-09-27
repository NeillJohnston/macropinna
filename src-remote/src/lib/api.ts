// Server communication

export interface RegisterRequest {
    device_name: string;
}

export interface RegisterResponse {
    uuid: string;
    code: string;
}

export interface AccessResponse {
    jwt?: string;
}

export type RemoteControlEvent = {
    DPad: 'Up' | 'Down' | 'Left' | 'Right' | 'Enter' | 'Exit';
}

// A class that sends events over WebSocket
export class EventSocket {
    constructor(
        public ws: WebSocket,
        public onClose: () => void
    ) {
        ws.addEventListener('close', onClose);
    }

    send(event: RemoteControlEvent) {
        this.ws.send(JSON.stringify(event));
    }
}

class RequestBuilder {
    constructor(
        private _url: string = '',
        private _method: string = '',
        private _body: string = '',
        private _headers: {[key: string]: string} = {}
    ) {}

    url(url: string): RequestBuilder {
        this._url = url;
        return this;
    }

    method(method: string): RequestBuilder {
        this._method = method;
        return this;
    }

    body(body: string): RequestBuilder {
        this._body = body;
        return this;
    }

    json(json: any): RequestBuilder {
        this._body = JSON.stringify(json);
        this.header('Content-Type', 'application/json');
        return this;
    }

    header(key: string, value: string): RequestBuilder {
        this._headers[key] = value;
        return this;
    }

    async response(): Promise<Response> {
        return fetch(
            this._url,
            {
                method: this._method,
                body: this._body,
                headers: this._headers
            }
        )
    }
}

export const register = async (
    request: RegisterRequest,
    onInit: (code: string) => void,
): Promise<AccessResponse> => {
    const res1 = await new RequestBuilder()
        .url('/api/register')
        .method('POST')
        .json(request)
        .response();
    const { uuid, code } = await res1.json() as RegisterResponse;

    onInit(code);

    const res2 = await new RequestBuilder()
        .url(`/api/register/${uuid}`)
        .method('POST')
        .response();
    return res2.json() as AccessResponse;
}

export const connect = async (jwt: string, onClose: () => void): Promise<EventSocket> => {
    // Directly from https://stackoverflow.com/a/47472874, thank you user Eadz
    let url = new URL(`/api/ws/${jwt}`, window.location.href);
    url.protocol = url.protocol.replace('http', 'ws');
    
    const ws = new WebSocket(url.href);

    ws.addEventListener('open', (event) => {
        console.log(event);
    });

    return new Promise((resolve, reject) => {
        ws.addEventListener('open', () => {
            resolve(new EventSocket(ws, onClose));
        });
    });
}