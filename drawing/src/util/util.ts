import { Plan } from "../models";

export function backendPlanToBabylonPlan(plan: Plan): Plan {
    return backendObjToBabylonObj(plan) as Plan;
}

function backendObjToBabylonObj(obj: any): any {
    let res: any;

    if (Array.isArray(obj)){
        res = [...obj];
    } else if (typeof obj === 'object') {
        res = {...obj};
    } else {
        res = obj;
    }

    if (objectIsPoint(obj)) {
        res.x = obj.x;
        res.y = obj.z;
        res.z = obj.y;
    } else {
        if (typeof obj !== 'string') {
            for (var prop in res) {
                if (Object.prototype.hasOwnProperty.call(obj, prop)) {
                    res[prop] = backendObjToBabylonObj(obj[prop]);
                }
            }
        }
    }

    return res;
}

function objectIsPoint(obj: any) {
    let buff = Object.keys(obj);
    return buff.includes('x') && buff.includes('y') && buff.includes('z') && buff.length === 3;
}
