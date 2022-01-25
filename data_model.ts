enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}

type Field = {
  isEmpty: boolean;
  color: Color;
};

type game = {
  id: string;
  startedAt: Date;
  finishedAt: Date | null;
  fields: Field[];
  players: [
    {
      playerId: string;
      color: Color;
      pawnsAtStart: number;
      home: Field[];
    }
  ];
};
