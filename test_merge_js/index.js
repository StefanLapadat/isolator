const martinez = require('martinez-polygon-clipping');
const geojson2svg = require('geojson-to-svg');

const gj1 = {
    "type": "Feature",
    "properties": {},
    "geometry": {
        "type": "Polygon",
        "coordinates": [
            [
                [0.0, 0.0],
                [9.447436571909622, -3.278100398057477],
                [12.725536969967099, 6.169336173852145],
                [3.278100398057477, 9.447436571909622],
                [0.0, 0.0]
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
                [10.103056651521117, -1.3886130836755528],
                [19.55049322343074, -4.66671348173303],
                [21.517353462265223, 1.0017484614127428],
                [12.069916890355604, 4.27984885947022],
                [10.103056651521117, -1.3886130836755528],
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