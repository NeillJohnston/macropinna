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