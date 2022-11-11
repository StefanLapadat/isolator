const martinez = require('martinez-polygon-clipping');
const geojson2svg = require('geojson-to-svg');

const gj1 = {
    "type": "Feature",
    "properties": {},
    "geometry": {
        "type": "Polygon",
        "coordinates": [
            [
                [-8.315300828559245, -5.554797217771496],
                [-4.1576504142796225, -2.777398608885748],
                [-8.04600846671967, 3.043311971105723],
                [-12.203658880999292, 0.2659133622199752],
                [-8.315300828559245, -5.554797217771496]
            ]
        ]
    }
}
const gj2 = {
    "type": "Feature",
    "properties": {},
    "geometry": {
        "type": "Polygon",
        "coordinates": [
            [
                [-12.203658880999292, 0.2659133622199752],
                [-3.8883580524400467, 5.820710579991471],
                [-6.665756661325794, 9.978360994271092],
                [-14.981057489885039, 4.423563776499597],
                [-12.203658880999292, 0.2659133622199752]
            ]
        ]
    }
}


const union = {
    "type": "Feature",
    "geometry": {
        "type": "Polygon",
        "coordinates": martinez.union(gj1.geometry.coordinates, gj2.geometry.coordinates)
    }
};

console.log(JSON.stringify(union, null, "  "));

console.log(geojson2svg()
    .data(gj1)
    .render());

console.log(geojson2svg()
    .data(gj2)
    .render());