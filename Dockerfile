# base node image
FROM node:18-bullseye-slim as base

# set for base and all layer that inherit from it
ENV NODE_ENV production

# Install all node_modules, including dev dependencies
FROM base as deps

WORKDIR /myapp

ADD .yarn ./.yarn
ADD package.json yarn.lock .yarnrc.yml ./
RUN yarn install

# Setup production node_modules
FROM base as production-deps

WORKDIR /myapp

ADD .yarn ./.yarn
ADD package.json yarn.lock .yarnrc.yml ./
RUN yarn workspaces focus --all --production

# Build the app
FROM base as build

WORKDIR /myapp

COPY --from=deps /myapp/node_modules /myapp/node_modules

ADD . .
RUN yarn run build

# Finally, build the production image with minimal footprint
FROM base

ENV DATA_PATH=/data
ENV PORT="8080"
ENV NODE_ENV="production"

WORKDIR /myapp

COPY --from=production-deps /myapp/node_modules /myapp/node_modules

COPY --from=build /myapp/build /myapp/build
COPY --from=build /myapp/public /myapp/public
COPY --from=build /myapp/package.json /myapp/package.json
COPY --from=build /myapp/start.sh /myapp/start.sh

ENTRYPOINT [ "./start.sh" ]
