import seedrandom from "seedrandom";
import dayjs from "dayjs";
import { Faker, en } from "@faker-js/faker";

function hash(text: string) {
  let hash = 0;
  if (text.length == 0) return hash;
  for (let i = 0; i < text.length; i++) {
    const character = text.charCodeAt(i);
    hash = (hash << 5) - hash + character;
    hash = hash & hash;
  }
  return hash;
}

export type DataGenerator = {
  generateFloat: () => number;
  generateDateInPast: () => string;
  generateDateInFuture: () => string;
  generateName: () => string;
  generateBranchName: () => string;
  generateCommitHash: () => string;
  generateId: () => string;
  pickRandomArrayElement: <T>(array: T[]) => T;
  randomPositiveInt: () => number;
  randomIntInRange: (min: number, max: number) => number;
};

export function getGenerator(seed: string): DataGenerator {
  const numberGenerator = seedrandom(seed);
  const fakerInstance = new Faker({ locales: { en } });
  fakerInstance.seed(hash(seed));

  const pickRandomArrayElement: DataGenerator["pickRandomArrayElement"] = (
    values
  ) => values[randomIntInRange(0, values.length - 1)];

  const generateDateInPast = () => {
    const date = dayjs()
      .subtract(randomIntInRange(1, 4), "month")
      .startOf("month");
    return date.format("YYYY-MM-DD");
  };

  const generateDateInFuture = () => {
    const date = dayjs().add(randomIntInRange(1, 4), "month").startOf("month");
    return date.format("YYYY-MM-DD");
  };

  const randomPositiveInt = () => Math.abs(numberGenerator.int32());
  const generateId = () => randomPositiveInt().toString();

  const randomIntInRange = (min: number, max: number) =>
    Math.round(numberGenerator() * (max - min)) + min;

  const generateName = () =>
    `${fakerInstance.person.firstName()} ${fakerInstance.person.lastName()}`;
  const generateBranchName = () => fakerInstance.git.branch();
  const generateCommitHash = () => fakerInstance.git.commitSha();

  return {
    generateFloat: numberGenerator,
    pickRandomArrayElement,
    generateDateInPast,
    generateDateInFuture,
    generateId,
    randomPositiveInt,
    randomIntInRange,
    generateName,
    generateBranchName,
    generateCommitHash,
  };
}
