import type { BracketStanding } from '@/stores/standings'

// Static fixture used when VITE_USE_MOCK_DATA=true, so the overlay can be
// styled/tweaked without a running backend. Deliberately includes varied
// name lengths and score widths to help spot CSS overflow issues.
export const mockBrackets: BracketStanding[] = [
  {
    name: 'Champions',
    groups: [
      {
        name: 'Group A',
        players: [
          { rank: 1, name: 'TheViper', setsWon: 3, setsLost: 0, mapsWon: 6, mapsLost: 1 },
          { rank: 2, name: 'Hera', setsWon: 2, setsLost: 1, mapsWon: 5, mapsLost: 3 },
          {
            rank: 3,
            name: 'MembTheGreatDestroyer',
            setsWon: 1,
            setsLost: 2,
            mapsWon: 3,
            mapsLost: 4,
          },
          { rank: 4, name: 'Yo', setsWon: 0, setsLost: 3, mapsWon: 1, mapsLost: 6 },
        ],
      },
      {
        name: 'Group B',
        players: [
          { rank: 1, name: 'Liereyy', setsWon: 3, setsLost: 0, mapsWon: 6, mapsLost: 0 },
          { rank: 2, name: 'DauT', setsWon: 2, setsLost: 1, mapsWon: 4, mapsLost: 3 },
          { rank: 3, name: 'Nicov', setsWon: 1, setsLost: 2, mapsWon: 3, mapsLost: 5 },
          { rank: 4, name: 'ACCM', setsWon: 0, setsLost: 3, mapsWon: 0, mapsLost: 6 },
        ],
      },
    ],
  },
  {
    name: 'Militia',
    groups: [
      {
        name: 'Group A',
        players: [
          {
            rank: 1,
            name: 'Some_Really_Long_Player_Name',
            setsWon: 2,
            setsLost: 0,
            mapsWon: 4,
            mapsLost: 1,
          },
          { rank: 2, name: 'Xx', setsWon: 1, setsLost: 1, mapsWon: 2, mapsLost: 2 },
          { rank: 3, name: 'Bob', setsWon: 0, setsLost: 2, mapsWon: 1, mapsLost: 4 },
        ],
      },
    ],
  },
  {
    name: 'Knights',
    groups: [
      {
        name: 'Group A',
        players: [
          { rank: 1, name: 'TaToH', setsWon: 3, setsLost: 0, mapsWon: 6, mapsLost: 2 },
          { rank: 2, name: 'Vinchester', setsWon: 2, setsLost: 1, mapsWon: 5, mapsLost: 4 },
          { rank: 3, name: 'JorDan_23', setsWon: 0, setsLost: 3, mapsWon: 2, mapsLost: 6 },
        ],
      },
      {
        name: 'Group B',
        players: [
          { rank: 1, name: 'Sotiris', setsWon: 3, setsLost: 0, mapsWon: 6, mapsLost: 1 },
          { rank: 2, name: 'MbL', setsWon: 1, setsLost: 2, mapsWon: 3, mapsLost: 5 },
          { rank: 3, name: 'Zuppa', setsWon: 1, setsLost: 2, mapsWon: 4, mapsLost: 4 },
        ],
      },
      {
        name: 'Group C',
        players: [
          { rank: 1, name: 'Daniel_AoE', setsWon: 2, setsLost: 1, mapsWon: 5, mapsLost: 3 },
          { rank: 2, name: 'F1re', setsWon: 2, setsLost: 1, mapsWon: 4, mapsLost: 3 },
          { rank: 3, name: 'GL.TheDragonAce', setsWon: 0, setsLost: 3, mapsWon: 1, mapsLost: 6 },
        ],
      },
    ],
  },
  {
    name: 'Pikemen',
    groups: [
      {
        name: 'Group A',
        players: [
          { rank: 1, name: 'Slam', setsWon: 2, setsLost: 0, mapsWon: 4, mapsLost: 1 },
          { rank: 2, name: 'Miguel', setsWon: 1, setsLost: 1, mapsWon: 3, mapsLost: 3 },
          { rank: 3, name: 'Hyu', setsWon: 0, setsLost: 2, mapsWon: 1, mapsLost: 4 },
        ],
      },
      {
        name: 'Group B',
        players: [
          { rank: 1, name: 'Dogao', setsWon: 2, setsLost: 0, mapsWon: 4, mapsLost: 0 },
          { rank: 2, name: 'Ecolo', setsWon: 1, setsLost: 1, mapsWon: 3, mapsLost: 3 },
          { rank: 3, name: 'Wam1zi', setsWon: 0, setsLost: 2, mapsWon: 1, mapsLost: 4 },
        ],
      },
      {
        name: 'Group C',
        players: [
          {
            rank: 1,
            name: 'Rubensztein_The_Second',
            setsWon: 2,
            setsLost: 0,
            mapsWon: 4,
            mapsLost: 2,
          },
          { rank: 2, name: 'Kasva', setsWon: 1, setsLost: 1, mapsWon: 3, mapsLost: 3 },
          { rank: 3, name: 'Fry', setsWon: 0, setsLost: 2, mapsWon: 2, mapsLost: 4 },
        ],
      },
      {
        name: 'Group D',
        players: [
          { rank: 1, name: 'Yasin', setsWon: 2, setsLost: 0, mapsWon: 4, mapsLost: 1 },
          { rank: 2, name: 'Tekken', setsWon: 1, setsLost: 1, mapsWon: 3, mapsLost: 3 },
          { rank: 3, name: 'Iju', setsWon: 0, setsLost: 2, mapsWon: 1, mapsLost: 4 },
        ],
      },
    ],
  },
]
