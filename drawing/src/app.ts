import {reloadApp as showBuilding} from './showBuilding';
import {reloadApp as showPlanExecution} from './plan_execution/showPlanExecution';
import { allInputFieldsCorrect } from './util/util';

populateInputFieldsFromLocalStorage();
showContent();

document.getElementById("mode")?.addEventListener('input', (event) => {
    localStorage.setItem("mode", getMode().toString());
    location.reload();
});

function getMode() {
    return parseInt((document.getElementById('mode') as any)?.value ?? localStorage.getItem('mode') ?? '1');
}

function populateInputFieldsFromLocalStorage() {
    (document.getElementById('mode') as any).value = localStorage.getItem('mode') ?? '1';
}

function showContent() {
    if(allInputFieldsCorrect()) {
        let mode = getMode();
        localStorage.setItem('mode', mode.toString());
        if (mode === 0) {
            showPlanExecution();
        } else if (mode === 1) {
            showBuilding();
        }
    } else {
        alert('nece moci ove noci')
    }
}