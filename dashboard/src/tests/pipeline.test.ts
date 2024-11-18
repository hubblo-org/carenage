import { render, screen } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';
import Pipeline from '../pipeline.svelte';

describe('pipeline test suite', () => {
	const pipeline = 'Pipeline #31';
	it('displays a heading with the executed pipeline number', () => {
		render(Pipeline);
		const pipelineHeading = screen.getByRole('heading', { name: pipeline });
		expect(pipelineHeading).toBeVisible();
	});
});
