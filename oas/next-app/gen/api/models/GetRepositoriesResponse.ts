/* tslint:disable */
/* eslint-disable */
/**
 * Demo NextJs
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import type { GetRepositoriesResponseData } from './GetRepositoriesResponseData';
import {
    GetRepositoriesResponseDataFromJSON,
    GetRepositoriesResponseDataFromJSONTyped,
    GetRepositoriesResponseDataToJSON,
} from './GetRepositoriesResponseData';

/**
 * 
 * @export
 * @interface GetRepositoriesResponse
 */
export interface GetRepositoriesResponse {
    /**
     * 
     * @type {GetRepositoriesResponseData}
     * @memberof GetRepositoriesResponse
     */
    data: GetRepositoriesResponseData;
    /**
     * 
     * @type {string}
     * @memberof GetRepositoriesResponse
     */
    type: string;
}

/**
 * Check if a given object implements the GetRepositoriesResponse interface.
 */
export function instanceOfGetRepositoriesResponse(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "data" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function GetRepositoriesResponseFromJSON(json: any): GetRepositoriesResponse {
    return GetRepositoriesResponseFromJSONTyped(json, false);
}

export function GetRepositoriesResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetRepositoriesResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'data': GetRepositoriesResponseDataFromJSON(json['data']),
        'type': json['type'],
    };
}

export function GetRepositoriesResponseToJSON(value?: GetRepositoriesResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'data': GetRepositoriesResponseDataToJSON(value.data),
        'type': value.type,
    };
}

