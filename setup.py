import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setuptools.setup(
        name="bundesliga-cli",
        version="0.1.0",
        author="Jannik Birk",
        author_email="birk.jannik@gmail.com",
        description="Show Bundesliga results on the command line",
        long_description=long_description,
        long_description_content_type="text/markdown",
        url="https://github.com/dernatsch/bundesliga-cli",
        packages=setuptools.find_packages(),
        classifiers=[
            "Programming Language :: Python :: 3",
            "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)",
            "Operating System :: OS Independent",
        ],
        python_requires='>=3.6',
        entry_points={
            'console_scripts': [
                'buli=buli:main',
            ],
        },
)
