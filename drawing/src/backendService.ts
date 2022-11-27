export interface Backend {
    get_plan(request_id: number, tile_length: number, tile_height: number, tile_width: number): Promise<Response>,
}

export class HttpBackend implements Backend {
    get_plan(request_id: number, tile_length: number, tile_height: number, tile_width: number): Promise<Response> {
        return fetch(`http://127.0.0.1:8000/generateplan?request_id=${request_id}&width=${tile_width}&height=${tile_height}&length=${tile_length}`);
    }
}
