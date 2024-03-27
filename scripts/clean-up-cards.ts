import { writeFile } from 'node:fs/promises';
import { cards as allCards } from './consts/cards';

import { cardPriceOverrides } from './overrides/card-prices';

const cleanCardData: unknown[] = [];

for (const card of allCards) {
	const override = cardPriceOverrides.find(
		(override) => override.cardName === card.name,
	);

	if (override) {
		card.price = override.cardValue;
	} else if (card.price < 6) {
		card.price = 0;
	}

	cleanCardData.push(card);
}

await writeFile(
	'prices.txt',
	`export const cards = ${JSON.stringify(cleanCardData)}`,
);
