import React from 'react';

type ExampleProps = {
	getData: () => string;
};

const ExampleComponent: React.FC<ExampleProps> = ({ getData }) => {
	const data = React.useMemo(getData, [getData]);
	return <span>{data}</span>;
};
