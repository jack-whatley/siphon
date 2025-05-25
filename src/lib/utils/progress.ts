export class ProgressBar {
    min: number;
    max: number;
    current: number;

    constructor(max: number) {
        this.min = 0;
        this.max = max;
        this.current = this.min;
    }

    public increment() {
        if (!(this.current + 1 > this.max)) {
            this.current += 1;
        }
    }

    public reset() {
        this.current = 0;
    }
}
