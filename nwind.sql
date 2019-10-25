CREATE TABLE customers (
  id              INT NOT NULL PRIMARY KEY,
  last_name       VARCHAR(50) ,
  first_name      VARCHAR(50) ,
  email           VARCHAR(50) ,
  company         VARCHAR(50) ,
  phone           VARCHAR(25) ,
  address1        VARCHAR(150),
  address2        VARCHAR(150),
  city            VARCHAR(50) ,
  state           VARCHAR(50) ,
  postal_code     VARCHAR(15) ,
  country         VARCHAR(50) 
);

CREATE TABLE employees (
  id              INT NOT NULL PRIMARY KEY,
  last_name       VARCHAR(50) ,
  first_name      VARCHAR(50) ,
  email           VARCHAR(50) ,
  avatar          VARCHAR(250) ,
  job_title       VARCHAR(50) ,
  department      VARCHAR(50) ,
  manager_id      INT ,
  phone           VARCHAR(25) ,
  address1        VARCHAR(150),
  address2        VARCHAR(150),
  city            VARCHAR(50) ,
  state           VARCHAR(50) ,
  postal_code     VARCHAR(15) ,
  country         VARCHAR(50) 
);

CREATE TABLE orders (
  id              INT NOT NULL PRIMARY KEY,
  employee_id     INT ,
  customer_id     INT ,
  order_date      DATETIME ,
  shipped_date    DATETIME ,
  ship_name       VARCHAR(50) ,
  ship_address1   VARCHAR(150) ,
  ship_address2   VARCHAR(150) ,
  ship_city       VARCHAR(50) ,
  ship_state      VARCHAR(50) ,
  ship_postal_code VARCHAR(50) ,
  ship_country    VARCHAR(50) ,
  shipping_fee    DECIMAL(19,4) NULL DEFAULT '0.0000',
  payment_type    VARCHAR(50) ,
  paid_date       DATETIME ,
  order_status    VARCHAR(25),
  PRIMARY KEY (id)
);

CREATE TABLE order_details (
  order_id            INT NOT NULL PRIMARY KEY,
  product_id          INT ,
  quantity            DECIMAL(18,4) NOT NULL DEFAULT '0.0000',
  unit_price          DECIMAL(19,4) NULL DEFAULT '0.0000',
  discount            DOUBLE NOT NULL DEFAULT '0',
  order_detail_status VARCHAR(25),
  date_allocated      DATETIME
);

CREATE TABLE products (
  id              INT NOT NULL PRIMARY KEY,
  product_code    VARCHAR(25) ,
  product_name    VARCHAR(50) ,
  description     VARCHAR(250),
  standard_cost   DECIMAL(19,4) NULL DEFAULT '0.0000',
  list_price      DECIMAL(19,4) NOT NULL DEFAULT '0.0000',
  target_level    INT ,
  reorder_level   INT ,
  minimum_reorder_quantity INT ,
  quantity_per_unit VARCHAR(50) ,
  discontinued    TINYINT NOT NULL DEFAULT '0',
  category        VARCHAR(50)
);

insert into customers (id, last_name, first_name, email, company, phone, address1, address2, city, state, postal_code, country) values 
 (1, 'Gray', 'Clarence', 'cgray0@rambler.ru', 'Jetpulse', '1-(260)601-5114', '02937 Merrick Avenue', null, 'Fort Wayne', 'Indiana', '46805', 'United States')
,(2, 'Cooper', 'Emily', 'ecooper1@macromedia.com', 'Skippad', '1-(251)614-5034', '60 Forster Crossing', null, 'Mobile', 'Alabama', '36605', 'United States')
,(3, 'Wilson', 'George', 'gwilson2@xinhuanet.com', 'Riffpath', '1-(901)445-9881', '52 Browning Center', null, 'Memphis', 'Tennessee', '38181', 'United States')
,(4, 'Mcdonald', 'Michael', 'mmcdonald3@twitter.com', 'Feedfire', '1-(419)743-7314', '85093 Jackson Park', null, 'Toledo', 'Ohio', '43610', 'United States')
,(5, 'Tucker', 'Lori', 'ltucker4@etsy.com', 'Oyondu', '1-(202)381-2663', '0706 Heffernan Pass', null, 'Washington', 'District of Columbia', '20380', 'United States')
,(6, 'Hansen', 'Lois', 'lhansen5@stumbleupon.com', 'Yozio', '1-(315)385-6866', '2 Ruskin Pass', null, 'Syracuse', 'New York', '13251', 'United States')
,(7, 'Grant', 'Frances', 'fgrant6@yale.edu', 'Eire', '1-(863)799-9068', '61402 Morning Court', null, 'Lakeland', 'Florida', '33811', 'United States')
,(8, 'Lewis', 'Catherine', 'clewis7@unesco.org', 'Realpoint', '1-(915)208-4997', '775 Messerschmidt Junction', null, 'El Paso', 'Texas', '88563', 'United States')
,(9, 'Rogers', 'Edward', 'erogers8@guardian.co.uk', 'Quinu', '1-(971)934-2404', '7074 Montana Place', null, 'Portland', 'Oregon', '97271', 'United States')
,(10, 'Owens', 'Tina', 'towens9@earthlink.net', 'Blogtag', '1-(712)989-9002', '59039 Sachtjen Street', null, 'Sioux City', 'Iowa', '51105', 'United States')
,(11, 'Peterson', 'Fred', 'fpetersona@home.pl', 'Meedoo', '1-(202)228-9380', '55 Knutson Street', null, 'Washington', 'District of Columbia', '20099', 'United States')
,(12, 'Cox', 'Jason', 'jcoxb@slashdot.org', 'Tagcat', '1-(651)991-0172', '19 Hoffman Avenue', null, 'Saint Paul', 'Minnesota', '55114', 'United States')
,(13, 'Perez', 'Paul', 'pperezc@ask.com', 'Kazu', '1-(405)290-7944', '6336 Continental Circle', null, 'Oklahoma City', 'Oklahoma', '73167', 'United States')
,(14, 'Duncan', 'Anthony', 'aduncand@disqus.com', 'Aibox', '1-(727)705-8400', '17794 Northport Point', null, 'Clearwater', 'Florida', '33763', 'United States')
,(15, 'Graham', 'Sean', 'sgrahame@google.com.br', 'Tekfly', '1-(901)443-0731', '47493 Reinke Drive', null, 'Memphis', 'Tennessee', '38131', 'United States')
,(16, 'Simpson', 'Jose', 'jsimpsonf@youtube.com', 'Youspan', '1-(518)381-4659', '498 Carey Lane', null, 'Albany', 'New York', '12262', 'United States')
,(17, 'Simmons', 'Frances', 'fsimmonsg@merriam-webster.com', 'Feednation', '1-(251)473-1327', '44102 Montana Crossing', null, 'Mobile', 'Alabama', '36641', 'United States')
,(18, 'Schmidt', 'John', 'jschmidth@vinaora.com', 'Topicblab', '1-(202)945-2079', '384 Goodland Pass', null, 'Washington', 'District of Columbia', '20210', 'United States')
,(19, 'West', 'Roger', 'rwesti@cornell.edu', 'Skilith', '1-(716)707-3907', '74 Nancy Pass', null, 'Buffalo', 'New York', '14215', 'United States')
,(20, 'Fields', 'Jesse', 'jfieldsj@shop-pro.jp', 'Vinte', '1-(617)810-9493', '602 Laurel Park', null, 'Boston', 'Massachusetts', '02203', 'United States')
,(21, 'Patterson', 'Kathy', 'kpattersonk@simplemachines.org', 'Brainverse', '1-(806)803-9725', '67063 Spenser Junction', null, 'Amarillo', 'Texas', '79188', 'United States')
,(22, 'Bell', 'Stephanie', 'sbelll@gravatar.com', 'DabZ', '1-(801)924-1975', '05 Gina Crossing', null, 'Salt Lake City', 'Utah', '84145', 'United States')
,(23, 'Turner', 'Martha', 'mturnerm@hp.com', 'Yadel', '1-(225)880-9134', '97789 Scofield Point', null, 'Baton Rouge', 'Louisiana', '70894', 'United States')
,(24, 'Ross', 'Richard', 'rrossn@weebly.com', 'Voolith', '1-(415)690-1916', '19 Briar Crest Alley', null, 'San Francisco', 'California', '94164', 'United States')
,(25, 'Stephens', 'Stephanie', 'sstephenso@timesonline.co.uk', 'BlogXS', '1-(501)827-1408', '63291 Huxley Drive', null, 'North Little Rock', 'Arkansas', '72118', 'United States')
,(26, 'Sanchez', 'Diane', 'dsanchezp@sun.com', 'Tagtune', '1-(315)979-6793', '13 Oneill Junction', null, 'Syracuse', 'New York', '13224', 'United States')
,(27, 'Lawrence', 'Douglas', 'dlawrenceq@flickr.com', 'Browsetype', '1-(610)622-4045', '83224 Mallard Center', null, 'Philadelphia', 'Pennsylvania', '19109', 'United States')
,(28, 'Hansen', 'Lori', 'lhansenr@patch.com', 'Einti', '1-(915)266-1989', '48410 Westport Circle', null, 'El Paso', 'Texas', '88546', 'United States')
,(29, 'Medina', 'Robert', 'rmedinas@uiuc.edu', 'Tambee', '1-(216)752-7216', '0562 Muir Trail', null, 'Cleveland', 'Ohio', '44197', 'United States')
,(30, 'Harrison', 'Shawn', 'sharrisont@webnode.com', 'Youopia', '1-(205)595-8085', '20 Mayer Center', null, 'Birmingham', 'Alabama', '35205', 'United States')
,(31, 'Alexander', 'Julia', 'jalexanderu@networksolutions.com', 'Oodoo', '1-(864)349-2185', '61859 West Point', null, 'Greenville', 'South Carolina', '29610', 'United States')
,(32, 'Stanley', 'Victor', 'vstanleyv@wikipedia.org', 'Centimia', '1-(561)507-7317', '08229 Glacier Hill Avenue', null, 'Boynton Beach', 'Florida', '33436', 'United States')
,(33, 'Wheeler', 'Dennis', 'dwheelerw@indiatimes.com', 'Dynabox', '1-(419)889-2883', '409 American Ash Plaza', null, 'Lima', 'Ohio', '45807', 'United States')
,(34, 'Frazier', 'Rachel', 'rfrazierx@sfgate.com', 'Skivee', '1-(314)249-1796', '7 Waxwing Court', null, 'Saint Louis', 'Missouri', '63131', 'United States')
,(35, 'Brooks', 'Shawn', 'sbrooksy@mediafire.com', 'Omba', '1-(971)854-8478', '614 Monica Terrace', null, 'Portland', 'Oregon', '97240', 'United States')
,(36, 'Coleman', 'Frank', 'fcolemanz@un.org', 'Agimba', '1-(318)446-4337', '4 Browning Park', null, 'Shreveport', 'Louisiana', '71151', 'United States')
,(37, 'Wright', 'Frank', 'fwright10@flickr.com', 'Meedoo', '1-(504)852-2553', '03446 Sheridan Trail', null, 'New Orleans', 'Louisiana', '70183', 'United States')
,(38, 'Perkins', 'Tina', 'tperkins11@va.gov', 'Rhybox', '1-(571)497-0755', '62 Grayhawk Lane', null, 'Fairfax', 'Virginia', '22036', 'United States')
,(39, 'Griffin', 'Ruth', 'rgriffin12@pcworld.com', 'Trudoo', '1-(504)312-7448', '29701 Jenifer Junction', null, 'Metairie', 'Louisiana', '70033', 'United States')
,(40, 'Sanchez', 'Sean', 'ssanchez13@bandcamp.com', 'Chatterpoint', '1-(316)535-7647', '10 Stuart Road', null, 'Wichita', 'Kansas', '67260', 'United States')
,(41, 'Harris', 'Linda', 'lharris14@ask.com', 'Blogpad', '1-(913)258-6547', '17635 Elka Alley', null, 'Shawnee Mission', 'Kansas', '66205', 'United States')
,(42, 'Wood', 'George', 'gwood15@imdb.com', 'Feednation', '1-(865)482-7169', '210 Anhalt Drive', null, 'Knoxville', 'Tennessee', '37924', 'United States')
,(43, 'Bennett', 'Nancy', 'nbennett16@networksolutions.com', 'Yodo', '1-(210)749-3167', '97861 Harper Pass', null, 'San Antonio', 'Texas', '78260', 'United States')
,(44, 'Pierce', 'Jesse', 'jpierce17@cyberchimps.com', 'Divanoodle', '1-(501)518-2300', '0 Columbus Terrace', null, 'Little Rock', 'Arkansas', '72231', 'United States')
,(45, 'Larson', 'Jerry', 'jlarson18@ustream.tv', 'Kazu', '1-(916)702-0977', '0422 Merry Court', null, 'Sacramento', 'California', '95852', 'United States')
,(46, 'Williams', 'Norma', 'nwilliams19@spiegel.de', 'Yacero', '1-(562)631-4036', '5779 Burning Wood Crossing', null, 'Long Beach', 'California', '90847', 'United States')
,(47, 'Kelley', 'Louis', 'lkelley1a@wsj.com', 'Linkbuzz', '1-(432)286-2200', '04321 Northport Pass', null, 'Odessa', 'Texas', '79764', 'United States')
,(48, 'Mason', 'Ruby', 'rmason1b@census.gov', 'Trudeo', '1-(763)835-7627', '99219 5th Place', null, 'Monticello', 'Minnesota', '55565', 'United States')
,(49, 'Carr', 'Nicole', 'ncarr1c@yellowbook.com', 'Plajo', '1-(330)154-9245', '41 Monica Parkway', null, 'Youngstown', 'Ohio', '44511', 'United States')
,(50, 'Franklin', 'Michelle', 'mfranklin1d@ucoz.com', 'Wikivu', '1-(405)751-8906', '62 Rutledge Alley', null, 'Oklahoma City', 'Oklahoma', '73147', 'United States')
,(51, 'Hawkins', 'Larry', 'lhawkins1e@typepad.com', 'Tagpad', '1-(240)796-9270', '48 Columbus Road', null, 'Hagerstown', 'Maryland', '21747', 'United States')
,(52, 'Adams', 'Lillian', 'ladams1f@smugmug.com', 'Avamm', '1-(419)394-2363', '69708 West Avenue', null, 'Toledo', 'Ohio', '43605', 'United States')
,(53, 'Williamson', 'Janice', 'jwilliamson1g@sbwire.com', 'Wordware', '1-(212)576-0309', '311 Clemons Crossing', null, 'New York City', 'New York', '10090', 'United States')
,(54, 'Mitchell', 'Lisa', 'lmitchell1h@live.com', 'Trudoo', '1-(225)794-6979', '17242 Eagan Terrace', null, 'Baton Rouge', 'Louisiana', '70820', 'United States')
,(55, 'Lawrence', 'Anna', 'alawrence1i@list-manage.com', 'Pixope', '1-(818)505-8262', '51241 Tennessee Point', null, 'Santa Monica', 'California', '90405', 'United States')
,(56, 'Scott', 'Margaret', 'mscott1j@arizona.edu', 'Mynte', '1-(202)265-0994', '117 Arapahoe Crossing', null, 'Washington', 'District of Columbia', '20051', 'United States')
,(57, 'Harper', 'Timothy', 'tharper1k@loc.gov', 'Trudoo', '1-(212)348-5025', '137 Commercial Court', null, 'New York City', 'New York', '10175', 'United States')
,(58, 'Frazier', 'Mary', 'mfrazier1l@mapy.cz', 'Photobug', '1-(214)939-0299', '281 Briar Crest Way', null, 'Garland', 'Texas', '75049', 'United States')
,(59, 'Young', 'Margaret', 'myoung1m@ehow.com', 'Realmix', '1-(318)144-2666', '537 Memorial Way', null, 'Shreveport', 'Louisiana', '71105', 'United States')
,(60, 'Sullivan', 'Phyllis', 'psullivan1n@jiathis.com', 'Demivee', '1-(518)361-7505', '117 Onsgard Crossing', null, 'Albany', 'New York', '12227', 'United States')
,(61, 'Knight', 'Roy', 'rknight1o@qq.com', 'Meezzy', '1-(478)438-2599', '60 Kinsman Drive', null, 'Macon', 'Georgia', '31210', 'United States')
,(62, 'Ruiz', 'Andrea', 'aruiz1p@domainmarket.com', 'Fivespan', '1-(415)374-7204', '55455 Heffernan Plaza', null, 'San Francisco', 'California', '94142', 'United States')
,(63, 'Hayes', 'Heather', 'hhayes1q@odnoklassniki.ru', 'Twitternation', '1-(601)919-6350', '31 Banding Plaza', null, 'Jackson', 'Mississippi', '39296', 'United States')
,(64, 'Chapman', 'Roy', 'rchapman1r@theguardian.com', 'BlogXS', '1-(941)329-4488', '27487 Fair Oaks Crossing', null, 'Sarasota', 'Florida', '34233', 'United States')
,(65, 'Moore', 'Daniel', 'dmoore1s@google.co.uk', 'Twitterbridge', '1-(408)838-8747', '52377 Everett Court', null, 'San Jose', 'California', '95194', 'United States')
,(66, 'Day', 'Amy', 'aday1t@indiegogo.com', 'Brainverse', '1-(336)903-6679', '50107 Northport Circle', null, 'Winston Salem', 'North Carolina', '27157', 'United States')
,(67, 'Kelley', 'Jesse', 'jkelley1u@who.int', 'Zoomzone', '1-(304)183-6041', '31740 Bayside Trail', null, 'Huntington', 'West Virginia', '25711', 'United States')
,(68, 'Andrews', 'Andrea', 'aandrews1v@patch.com', 'Fanoodle', '1-(206)996-3516', '544 Lakewood Gardens Alley', null, 'Seattle', 'Washington', '98148', 'United States')
,(69, 'Willis', 'Robert', 'rwillis1w@si.edu', 'Tazz', '1-(678)738-3382', '0998 Bellgrove Circle', null, 'Duluth', 'Georgia', '30195', 'United States')
,(70, 'Reid', 'Daniel', 'dreid1x@miitbeian.gov.cn', 'Babbleblab', '1-(786)539-0220', '2 New Castle Avenue', null, 'Miami', 'Florida', '33111', 'United States')
,(71, 'Roberts', 'Louise', 'lroberts1y@cpanel.net', 'Skippad', '1-(316)418-5043', '598 Mayfield Road', null, 'Wichita', 'Kansas', '67210', 'United States')
,(72, 'Harrison', 'Margaret', 'mharrison1z@biglobe.ne.jp', 'Zoovu', '1-(775)625-5357', '636 Maple Point', null, 'Reno', 'Nevada', '89550', 'United States')
,(73, 'Chapman', 'Peter', 'pchapman20@eventbrite.com', 'Linkbuzz', '1-(574)707-4449', '1 Vermont Plaza', null, 'South Bend', 'Indiana', '46620', 'United States')
,(74, 'Tucker', 'Willie', 'wtucker21@hubpages.com', 'Eazzy', '1-(814)593-2963', '3 Dryden Trail', null, 'Erie', 'Pennsylvania', '16505', 'United States')
,(75, 'Bradley', 'Adam', 'abradley22@cnet.com', 'Yabox', '1-(903)334-0845', '15 Veith Center', null, 'Tyler', 'Texas', '75705', 'United States')
,(76, 'Armstrong', 'Juan', 'jarmstrong23@archive.org', 'Gevee', '1-(916)285-8157', '95 Claremont Crossing', null, 'Sacramento', 'California', '95865', 'United States')
,(77, 'Payne', 'Kenneth', 'kpayne24@com.com', 'Skinte', '1-(770)150-8423', '75 Beilfuss Court', null, 'Alpharetta', 'Georgia', '30022', 'United States')
,(78, 'Jacobs', 'Louis', 'ljacobs25@intel.com', 'Photolist', '1-(404)386-3449', '542 Browning Park', null, 'Atlanta', 'Georgia', '30343', 'United States')
,(79, 'Cooper', 'Marilyn', 'mcooper26@whitehouse.gov', 'Yotz', '1-(520)875-3539', '03 Transport Point', null, 'Tucson', 'Arizona', '85710', 'United States')
,(80, 'Watkins', 'William', 'wwatkins27@pbs.org', 'Brightbean', '1-(719)568-3678', '0316 La Follette Drive', null, 'Colorado Springs', 'Colorado', '80920', 'United States')
,(81, 'Baker', 'Douglas', 'dbaker28@exblog.jp', 'Voonix', '1-(619)121-6070', '99 Kedzie Terrace', null, 'Chula Vista', 'California', '91913', 'United States')
,(82, 'Cunningham', 'Daniel', 'dcunningham29@oakley.com', 'Leexo', '1-(619)609-6206', '53540 Hallows Place', null, 'San Diego', 'California', '92137', 'United States')
,(83, 'Mitchell', 'Edward', 'emitchell2a@amazon.com', 'Jatri', '1-(502)427-5275', '3 Jana Junction', null, 'Louisville', 'Kentucky', '40280', 'United States')
,(84, 'Matthews', 'Adam', 'amatthews2b@live.com', 'Muxo', '1-(316)764-9074', '5 Arapahoe Plaza', null, 'Wichita', 'Kansas', '67220', 'United States')
,(85, 'Jacobs', 'Diane', 'djacobs2c@jimdo.com', 'Youspan', '1-(804)398-4775', '6 Moose Street', null, 'Richmond', 'Virginia', '23285', 'United States')
,(86, 'Frazier', 'Mary', 'mfrazier2d@forbes.com', 'Zoomdog', '1-(716)502-4243', '713 Continental Trail', null, 'Buffalo', 'New York', '14210', 'United States')
,(87, 'Howard', 'Anthony', 'ahoward2e@disqus.com', 'Katz', '1-(805)800-7968', '006 Elka Center', null, 'Bakersfield', 'California', '93311', 'United States')
,(88, 'Kelly', 'Anna', 'akelly2f@unc.edu', 'Trilia', '1-(804)117-9707', '292 Browning Circle', null, 'Richmond', 'Virginia', '23228', 'United States')
,(89, 'Clark', 'Kathy', 'kclark2g@businesswire.com', 'Gigabox', '1-(831)755-4869', '45 Walton Parkway', null, 'Salinas', 'California', '93907', 'United States')
,(90, 'Carpenter', 'Gerald', 'gcarpenter2h@foxnews.com', 'Yakidoo', '1-(213)344-9608', '84560 Stephen Parkway', null, 'Los Angeles', 'California', '90055', 'United States')
,(91, 'Hart', 'Albert', 'ahart2i@scribd.com', 'Jaloo', '1-(415)201-5281', '0 Washington Place', null, 'Oakland', 'California', '94611', 'United States')
,(92, 'Young', 'Julia', 'jyoung2j@craigslist.org', 'Oloo', '1-(601)660-1203', '0623 Gale Drive', null, 'Jackson', 'Mississippi', '39210', 'United States')
,(93, 'Pierce', 'Tammy', 'tpierce2k@psu.edu', 'Ntags', '1-(703)772-3155', '501 Leroy Parkway', null, 'Washington', 'District of Columbia', '20041', 'United States')
,(94, 'Washington', 'Ronald', 'rwashington2l@drupal.org', 'Livetube', '1-(202)828-6093', '6220 Del Sol Alley', null, 'Washington', 'District of Columbia', '20551', 'United States')
,(95, 'Carter', 'Alan', 'acarter2m@simplemachines.org', 'Centizu', '1-(952)234-1678', '043 Sunnyside Center', null, 'Young America', 'Minnesota', '55551', 'United States')
,(96, 'Freeman', 'Stephanie', 'sfreeman2n@redcross.org', 'Feednation', '1-(504)126-8245', '1120 Veith Parkway', null, 'New Orleans', 'Louisiana', '70165', 'United States')
,(97, 'Price', 'Sara', 'sprice2o@irs.gov', 'Izio', '1-(415)668-5714', '96394 Bonner Hill', null, 'San Francisco', 'California', '94116', 'United States')
,(98, 'Parker', 'Emily', 'eparker2p@4shared.com', 'Skalith', '1-(330)637-4894', '0 Old Shore Center', null, 'Akron', 'Ohio', '44310', 'United States')
,(99, 'Chavez', 'Jeremy', 'jchavez2q@businessweek.com', 'Topicware', '1-(515)769-2045', '8584 Jay Street', null, 'Des Moines', 'Iowa', '50335', 'United States')
,(100, 'Kim', 'Pamela', 'pkim2r@stumbleupon.com', 'Photolist', '1-(510)144-4318', '3688 Gerald Trail', null, 'Sacramento', 'California', '95823', 'United States');
