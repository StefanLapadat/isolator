const martinez = require('martinez-polygon-clipping');
// const geojson2svg = require('geojson-to-svg');

const gj1 = {
    "type": "Feature",
    "properties": {},
    "geometry": {
        "type": "Polygon",
        "coordinates": [
            [
                [0.0, 0.0],
                [9.447436, -3.2781005],
                [12.725537, 6.1693363],
                [3.2781005, 9.447436],
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
                [10.103057, -1.3886131],
                [19.550493, -4.6667137],
                [21.517353, 1.0017484],
                [12.069917, 4.279849],
                [10.103057, -1.3886131]
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

// console.log(geojson2svg()
//     .data(gj1)
//     .render());

// console.log(geojson2svg()
//     .data(gj2)
//     .render());