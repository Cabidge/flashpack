export class Deck {
    size: number;
    unseen: number[] = [];

    public constructor(size: number) {
        if (size < 1) {
            throw 'Deck size must be at least 1';
        }

        this.size = size;
    }

    public next(excludeIndex?: number): number {
        if (this.unseen.length == 0) {
            this.unseen = Array.from(Array(this.size).keys()).filter((n) => n !== excludeIndex);
        }

        const index = Math.floor(Math.random() * this.unseen.length);

        const output = this.unseen[index];

        const end = this.unseen.pop();
        if (index < this.unseen.length && end !== undefined) {
            this.unseen[index] = end;
        }

        return output;
    }
}
