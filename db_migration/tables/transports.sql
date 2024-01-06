use `eco-friend`;
drop table if exists Transport ;
CREATE TABLE Transport (
	id integer AUTO_INCREMENT not null,
    tName varchar(300),
    tType varchar(100),
    fuel  varchar (100),
    unit  varchar (100),
    region varchar(100),
    efFactor DECIMAL(10, 6),
    PRIMARY KEY(id)
);