class Flock {
    seagulls: number;

    constructor(n: number) {
        this.seagulls = n;
    }

    conjoin(other: Flock): Flock {
        this.seagulls += other.seagulls;
        return this;
    }

    breed(other: Flock): Flock {
        this.seagulls *= other.seagulls;
        return this;
    }
}

const flockA = new Flock(4);
const flockB = new Flock(2);
const flockC = new Flock(0);

const result = flockA
    .conjoin(flockC)
    .breed(flockB)
    .conjoin(flockA.breed(flockB))
    .seagulls;

console.log(result);
