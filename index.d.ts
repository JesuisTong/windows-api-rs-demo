/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function getProcessExists(processName: string, exact: boolean): boolean
export function killProcess(processName: string): void
export function readRegistry(regKeyRoot: 0 | 1 | 2, regPath: string, regKeyName: string): unknown
export function writeRegistry(regKeyRoot: 0 | 1 | 2, regPath: string, regKeyName: string, regKeyValue: unknown): void
export function deleteRegistry(regKeyRoot: 0 | 1 | 2, regPath: string, regKeyName: string): void
export function showWindowByTitle(windowTitle: string): void
