Home Fairie
===========

Home Fairie is a set of lightweight utilities (fairies)
for various home environment monitoring, data logging
and automation tasks.

Features (so far..)
-------------------

* Zigbee2MQTT sensor data logging to PostgreSQL database

Setup
-----

Create database schema:

.. code-block:: sql

    CREATE TABLE device_meters (
        "timestamp" timestamp with time zone,
        name text,
        value double precision,
        dimensions jsonb,
        value_meta json
    );

    CREATE INDEX ON device_meters (name, timestamp);
    CREATE INDEX ON device_meters USING GIN (dimensions);


And run the application:

.. code-block:: text

    $ POSTGRESQL_URL=postgresql:/meters MQTT_URL=zigbee2mqtt-gateway.local MQTT_PORT=1883 cargo run
