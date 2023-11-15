export function minIndexByKey<T>(array: T[], key: (val: T, index: number) => number): number | undefined {
    if (array.length === 0) return undefined;

    const keys = array.map(key);
    const minKey = Math.min(...keys);
    const index = keys.indexOf(minKey);
    return index;
}